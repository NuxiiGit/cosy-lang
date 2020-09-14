pub mod syntax;

use syntax::*;

use libcosyc_concrete::syntax as concrete;
use libcosyc_diagnostics::error::{ Diagnostic, IssueTracker, ErrorLevel };

/// Type alias for issue tracking.
pub type Issues<'a> = &'a mut IssueTracker;

/// Provides an interface for desugaring concrete syntax into abstract syntax.
pub trait Desugar {
    /// The type to desugar into.
    type Out;

    /// Desugar `self` into the type `Out`.
    fn desugar(self, issues : Issues) -> Self::Out;
}

impl Desugar for concrete::Expr {
    type Out = Option<Expr>;
    fn desugar(self, issues : Issues) -> Self::Out {
        let span = self.span;
        let kind = match self.kind {
            concrete::ExprKind::Variable => ExprKind::Variable,
            concrete::ExprKind::Integral => ExprKind::Integral,
            concrete::ExprKind::Grouping { lparen, rparen, inner } => {
                if !lparen {
                    Diagnostic::from(&span)
                            .level(ErrorLevel::Warning)
                            .reason_str("missing opening parenthesis in grouping")
                            .note_str("consider adding `(` before this expression")
                            .report(issues);
                }
                if !rparen {
                    Diagnostic::from(&span)
                            .level(ErrorLevel::Warning)
                            .reason_str("missing closing parenthesis in grouping")
                            .note_str("consider adding `)` to complete this grouping")
                            .report(issues);
                }
                if let Some(expr) = inner {
                    return expr.desugar(issues);
                } else {
                    ExprKind::Empty
                }
            },
            concrete::ExprKind::Malformed => {
                Diagnostic::from(&span)
                        .level(ErrorLevel::Fatal)
                        .reason_str("unexpected symbol in expression")
                        .note_str("consider removing this symbol")
                        .report(issues);
                return None;
            }
        };
        Some(Expr { span, kind })
    }
}
