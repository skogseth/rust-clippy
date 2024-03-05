use rustc_lint::LateContext;
use clippy_utils::diagnostics::span_lint_and_help;
use rustc_span::Span;
use rustc_hir::intravisit::FnKind;
use rustc_hir::{GenericParamKind, LifetimeParamKind, FnDecl};

use super::INVISIBLE_LIFETIMES;

pub(super) fn check_fn<'tcx>(
    cx: &LateContext<'tcx>,
    kind: FnKind<'tcx>,
    _decl: &'tcx FnDecl<'_>,
    _span: Span,
) {
    if let Some(span) = should_emit(kind) {
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

fn should_emit<'tcx>(kind: FnKind<'tcx>) -> Option<Span> {
    // Only support free standing functions (for now)
    let FnKind::ItemFn(_, generics, _) = kind else { return None };

    for param in generics.params {
        if let GenericParamKind::Lifetime { kind: lifetime_kind } = param.kind {
            if let LifetimeParamKind::Elided = lifetime_kind {
                return Some(param.span);
            }
        }
    }

    None
}

