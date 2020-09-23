use libcosyc_diagnostics::source::Span;
use libcosyc_abstract::syntax::*;

use std::fmt;

/// Represents the state of the C code generator.
pub struct Codegen<'a> {
    out : &'a mut dyn fmt::Write
}
impl<'a> Codegen<'a> {
    /// Creates a new C code generator.
    pub fn new(out : &'a mut dyn fmt::Write) -> Self {
        Self { out }
    }

    /// Emits an expression of any kind.
    pub fn emit_expr(&mut self, expr : Expr) -> fmt::Result {
        let out = &mut self.out;
        let span = expr.span;
        match expr.kind {
            ExprKind::Variable => write!(out, "variable"),
            ExprKind::Integral => write!(out, "integral"),
            ExprKind::Empty => unimplemented!()
        }
    }
}
