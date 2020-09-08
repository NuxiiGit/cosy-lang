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
        match self.kind {
            concrete::ExprKind::Terminal(option) => {
                if let Some(terminal_kind) = option {
                    let kind = match terminal_kind {
                        concrete::TerminalKind::Variable => ExprKind::Variable,
                        concrete::TerminalKind::Integral => ExprKind::Value(ValueKind::Integral)
                    };
                    Some(Expr { span, kind })
                } else {
                    Diagnostic::from(&span)
                            .level(ErrorLevel::Bug)
                            .reason(format!("malformed terminal expression"))
                            .report(issues);
                    None
                }
            }
        }
    }
}
