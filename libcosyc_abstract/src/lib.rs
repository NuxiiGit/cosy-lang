pub mod syntax;

use syntax::*;

use libcosyc_concrete::syntax as concrete;
use libcosyc_diagnostics::error::{ Diagnostic, IssueTracker, ErrorLevel };

/// Represents the state of a desugar controller.
pub struct Desugar<'a> {
    issues : &'a mut IssueTracker
}
impl Desugar<'_> {
    /// Submits a diagnostic to the current issue tracker.
    pub fn report(&mut self, diagnostic : Diagnostic) {
        diagnostic.report(self.issues);
    }

    /// Desugars an expression.
    pub fn desugar_expr(&mut self, expr : concrete::Expr) -> Option<Expr> {
        let span = expr.span;
        let kind = match expr.kind {
            concrete::ExprKind::Variable => ExprKind::Variable,
            concrete::ExprKind::Integral => ExprKind::Integral,
            concrete::ExprKind::Grouping { lparen, rparen, inner } => {
                if !lparen {
                    self.report(Diagnostic::from(&span)
                            .level(ErrorLevel::Warning)
                            .reason_str("missing opening parenthesis in grouping")
                            .note_str("consider adding `(` before this expression"));
                }
                if !rparen {
                    self.report(Diagnostic::from(&span)
                            .level(ErrorLevel::Warning)
                            .reason_str("missing closing parenthesis in grouping")
                            .note_str("consider adding `)` to complete this grouping"));
                }
                if let Some(expr) = inner {
                    return self.desugar_expr(*expr);
                } else {
                    ExprKind::Empty
                }
            },
            concrete::ExprKind::Malformed => {
                self.report(Diagnostic::from(&span)
                        .level(ErrorLevel::Fatal)
                        .reason_str("unexpected symbol in expression")
                        .note_str("consider removing this symbol"));
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
