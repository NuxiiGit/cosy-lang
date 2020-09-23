use libcosyc_abstract::syntax::*;
use libcosyc_diagnostics::{
    Session,
    source::Span,
    error::{ Diagnostic, IssueTracker, ErrorLevel }
};


use std::fmt;

/// Represents the state of the C code generator.
pub struct Codegen<'a> {
    out : &'a mut dyn fmt::Write
}
impl<'a> Codegen<'a> {
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
impl<'a> From<&'a mut Session> for Codegen<'a> {
    fn from(sess : &'a mut Session) -> Self {
        let out = &mut sess.out;
        let issues = &mut sess.issues;
        Self { out }
    }
}
