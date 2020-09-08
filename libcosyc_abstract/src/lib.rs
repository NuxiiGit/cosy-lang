pub mod syntax;

use libcosyc_concrete::syntax as concrete;
use libcosyc_source::Span;
use libcosyc_diagnostics::{ Diagnostic, IssueTracker, ErrorLevel };

/// Provides an interface for desugaring concrete syntax into abstract syntax.
trait Desugar<R> {
    /// Desugar `self` into the type `R`.
    fn desugar(self, issues : &mut IssueTracker) -> R;
}

impl Desugar<syntax::Expr> for concrete::ExprKind {
    fn desugar(self, issues : &mut IssueTracker) -> syntax::Expr {
        unimplemented!()
    }
}
