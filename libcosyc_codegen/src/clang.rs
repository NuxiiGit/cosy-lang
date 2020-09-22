use libcosyc_diagnostics::source::Span;
use libcosyc_abstract::syntax::*;

use std::fmt;

/// Represents the state of the C code generator.
pub struct Codegen<'a> {
    out : &'a mut dyn fmt::Write
}
impl Codegen<'_> {
    /// Emits an expression of any kind.
    pub fn emit_expr(&mut self, expr : Expr) -> fmt::Result {
        let span = expr.span;
        let kind = match expr.kind {
            ExprKind::Variable => unimplemented!(),
            ExprKind::Integral => unimplemented!(),
            ExprKind::Empty => unimplemented!()
        };
        Ok(())
    }
}
impl<'a> From<&'a mut dyn fmt::Write> for Codegen<'a> {
    fn from(out : &'a mut dyn fmt::Write) -> Self {
        Self { out }
    }
}
