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
        let kind = match self.kind {
            concrete::ExprKind::Variable => ExprKind::Variable,
            concrete::ExprKind::Integral => ExprKind::Integral,
            concrete::ExprKind::Grouping { unclosed, inner } => {
                if unclosed {
                    Diagnostic::from(&span)
                            .level(ErrorLevel::Warning)
                            .reason(format!("missing closing parenthesis in expression"))
                            .report(issues);
                }
                return inner.desugar(issues);
            },
            concrete::ExprKind::Malformed => {
                Diagnostic::from(&span)
                        .level(ErrorLevel::Fatal)
                        .reason(format!("malformed expression"))
                        .report(issues);
                return None;
            }
        };
        Some(Expr { span, kind })
    }
}
