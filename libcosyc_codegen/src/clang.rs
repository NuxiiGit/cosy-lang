use libcosyc_abstract::syntax::*;
use libcosyc_diagnostics::{
    Session,
    source::Span,
    error::{ Diagnostic, IssueTracker, ErrorLevel }
};

use crate::ident::{ Identifier, NameTable };

use std::fmt;
use fmt::Write;

/// Represents the state of the C code generator.
pub struct Codegen<'a> {
    src : &'a str,
    out : &'a mut String,
    issues : &'a IssueTracker,
    name_table : NameTable<'a>
}
impl<'a> Codegen<'a> {
    /// Emits an expression of any kind.
    pub fn emit_expr(&mut self, expr : Expr) -> fmt::Result {
        let span = expr.span;
        match expr.kind {
            ExprKind::Variable => self.emit_ident(span),
            ExprKind::Integral => unimplemented!(),
            ExprKind::Empty => unimplemented!()
        }
    }

    /// Emits an identifier. Non-valid C identifier symbols are converted into hexidecimal.
    pub fn emit_ident(&mut self, span : Span) -> fmt::Result {
        let ident = &self.src[span.begin..span.end];
        write!(self.out, "{}", ident)?;
        Ok(())
    }
}
impl<'a> From<&'a mut Session> for Codegen<'a> {
    fn from(sess : &'a mut Session) -> Self {
        let src = &sess.src;
        let out = &mut sess.out;
        let issues = &mut sess.issues;
        let name_table = NameTable::new();
        Self { src, out, issues, name_table }
    }
}
