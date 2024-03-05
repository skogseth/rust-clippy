use rustc_ast::ast::*;
use rustc_lint::{EarlyContext, EarlyLintPass};
use rustc_session::declare_lint_pass;
use clippy_utils::diagnostics::span_lint_and_help;
use rustc_ast::visit::FnKind;
use rustc_span::Span;

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
    pub FOO_FUNCTIONS,
    pedantic,
    "default lint description"
}

declare_lint_pass!(FooFunctions => [FOO_FUNCTIONS]);

impl EarlyLintPass for FooFunctions {
    fn check_fn(&mut self, cx: &EarlyContext<'_>, fn_kind: FnKind<'_>, span: Span, _: NodeId) {
        if is_fn_named_foo(fn_kind) {
            span_lint_and_help(
                cx,
                FOO_FUNCTIONS,
                span,
                "function named `foo`",
                None,
                "consider using a more meaningful name",
            );
        }
    }
}

fn is_fn_named_foo(fn_kind: FnKind<'_>) -> bool {
    match fn_kind {
        FnKind::Fn(_, ident, ..) => {
            ident.name.as_str() == "foo"
        }
        // Ignore closures
        FnKind::Closure(..) => false,
    }
}
