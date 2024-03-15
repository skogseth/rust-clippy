use clippy_utils::diagnostics::span_lint_and_help;
use rustc_hir::*;
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::declare_lint_pass;
use rustc_span::symbol::kw;

declare_clippy_lint! {
    /// ### What it does
    ///
    /// ### Why is this bad?
    ///
    /// ### Example
    /// ```no_run
    /// // example code where clippy issues a warning
    /// ```
    /// Use instead:
    /// ```no_run
    /// // example code which does not raise clippy warning
    /// ```
    #[clippy::version = "1.78.0"]
    pub NON_HIERARCHICAL_CRATE_PATHS,
    restriction,
    "default lint description"
}

declare_lint_pass!(NonHierarchicalCratePaths => [NON_HIERARCHICAL_CRATE_PATHS]);

impl<'tcx> LateLintPass<'tcx> for NonHierarchicalCratePaths {
    fn check_path(&mut self, cx: &LateContext<'tcx>, path: &Path<'tcx>, _: HirId) {
        if path_is_non_hierarchical(path) {
            span_lint_and_help(
                cx,
                NON_HIERARCHICAL_CRATE_PATHS,
                path.span,
                "non hierarchical crate path",
                None,
                "structure your code in a hierarchical fashion",
            );
        }
    }
}

fn path_is_non_hierarchical(path: &Path<'_>) -> bool {
    path.segments
        .first()
        .is_some_and(|first_segment| matches!(first_segment.ident.name, kw::Crate | kw::Super))
}
