use rustc_lint::LateContext;
use clippy_utils::diagnostics::span_lint_and_help;
use rustc_span::Span;
use rustc_hir::intravisit::FnKind;
use rustc_hir::{GenericParamKind, LifetimeParamKind, FnDecl};

use super::INVISIBLE_LIFETIMES;

pub(super) fn check_fn<'tcx>(
    cx: &LateContext<'tcx>,
    kind: FnKind<'tcx>,
    decl: &'tcx FnDecl<'_>,
    _span: Span,
) {
    for span in emits(kind, decl) {
        span_lint_and_help(
            cx,
            INVISIBLE_LIFETIMES,
            span,
            "found elided lifetime",
            None,
            "explicitly annotate the lifetime",
        );
    }
}

fn emits<'tcx>(kind: FnKind<'tcx>, decl: &'tcx FnDecl<'_>) -> Vec<Span> {
    // Only support free standing functions (for now)
    let FnKind::ItemFn(_, generics, _) = kind else { return Vec::new() };

    let generics: Vec<_> = generics.params.into_iter().filter_map(|param| match param.kind {
        GenericParamKind::Lifetime { 
            kind: LifetimeParamKind::Elided,
        } => Some((param.hir_id, param.span)),
        _ => None,
    }).collect();

    let generic_hir_ids: Vec<_> = generics.iter().map(|(hir_id, _)| hir_id).collect();
    eprintln!("Generics: {generic_hir_ids:?}");

    let param_hir_ids: Vec<_> = decl.inputs.iter().map(|input| input.hir_id).collect();
    eprintln!("Params: {param_hir_ids:?}");

    if let rustc_hir::FnRetTy::Return(ty) = decl.output {
        eprintln!("Output: {:?}", ty.hir_id);
    }

    generics.into_iter().map(|(_, s)| s).collect()
}

