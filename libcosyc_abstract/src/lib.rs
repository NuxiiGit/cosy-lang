pub mod syntax;

use syntax::*;

use libcosyc_concrete::syntax as concrete;
use libcosyc_source::Span;
use libcosyc_diagnostics::{ Diagnostic, IssueTracker, ErrorLevel };

/// Provides an interface for desugaring concrete syntax into abstract syntax.
pub trait Desugar {
    /// The type to desugar into.
    type Out;

    /// Desugar `self` into the type `Out`.
    fn desugar(self, issues : &mut IssueTracker) -> Self::Out;
}

impl Desugar for concrete::Expr {
    type Out = Option<Expr>;
    fn desugar(self, issues : &mut IssueTracker) -> Self::Out {
        let span = self.span;
        if let Some(concrete_kind) = self.kind {
            let kind = match concrete_kind {
                concrete::ExprKind::Variable => ExprKind::Variable,
                _ => {
                    Diagnostic::from(&span)
                            .level(ErrorLevel::Bug)
                            .reason(format!("unsupported expression kind"))
                            .report(issues);
                    return None;
                }
            };
            Some(Expr { span, kind })
        } else {
            Diagnostic::from(&span)
                    .level(ErrorLevel::Fatal)
                    .reason(format!("malformed expression"))
                    .report(issues);
            None
        }
    }
}
