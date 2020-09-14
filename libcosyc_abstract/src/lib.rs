pub mod syntax;

use syntax::*;

use libcosyc_concrete::syntax as concrete;
use libcosyc_diagnostics::error::{ Diagnostic, IssueTracker, ErrorLevel };

/// Represents the state of a desugar controller.
pub struct Desugar<'a> {
    issues : &'a mut IssueTracker
}
impl Desugar<'_> {
    /// Desugars an expression.
    fn desugar(self, expr : concrete::Expr) -> Option<Expr> {
        let span = expr.span;
        let kind = match expr.kind {
            concrete::ExprKind::Variable => ExprKind::Variable,
            concrete::ExprKind::Integral => ExprKind::Integral,
            concrete::ExprKind::Grouping { lparen, rparen, inner } => {
                if !lparen {
                    Diagnostic::from(&span)
                            .level(ErrorLevel::Warning)
                            .reason_str("missing opening parenthesis in grouping")
                            .note_str("consider adding `(` before this expression")
                            .report(self.issues);
                }
                if !rparen {
                    Diagnostic::from(&span)
                            .level(ErrorLevel::Warning)
                            .reason_str("missing closing parenthesis in grouping")
                            .note_str("consider adding `)` to complete this grouping")
                            .report(self.issues);
                }
                if let Some(expr) = inner {
                    return self.desugar(*expr);
                } else {
                    ExprKind::Empty
                }
            },
            concrete::ExprKind::Malformed => {
                Diagnostic::from(&span)
                        .level(ErrorLevel::Fatal)
                        .reason_str("unexpected symbol in expression")
                        .note_str("consider removing this symbol")
                        .report(self.issues);
                return None;
            }
        };
        Some(Expr { span, kind })
    }
}
impl<'a> From<&'a mut IssueTracker> for Desugar<'a> {
    fn from(issues : &'a mut IssueTracker) -> Self {
        Self { issues }
    }
}
