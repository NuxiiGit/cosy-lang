use crate::ir;
use libcosyc_diagnostic::{
    source::Renderable,
    error::{ CompilerError, IssueTracker, Failable }
};
use libcosyc_parse::syntax as ast;

/// Manages the conversion of AST terms into IR instructions.
pub struct IRBuilder<'a> {
    src : &'a str,
    issues : &'a mut IssueTracker
}

impl Failable for IRBuilder<'_> {
    fn issues(&mut self) -> &mut IssueTracker {
        self.issues
    }
}

impl Renderable for IRBuilder<'_> {
    fn src(&self) -> &str {
        self.src
    }
}

impl<'a> IRBuilder<'a> {
    /// Creates a new instance from this issue tracker and source file.
    pub fn new(src : &'a str, issues : &'a mut IssueTracker) -> Self {
        Self { src, issues }
    }

    /// Generates instructions from AST terms.
    pub fn desugar(&mut self, term : ast::Term) -> Option<ir::Inst> {
        let span = term.span;
        let kind = match term.kind {
            ast::TermKind::Variable => ir::InstKind::Variable,
            ast::TermKind::Integral { radix } => ir::InstKind::Integral { radix },
            ast::TermKind::TypeAnno { value, datatype } => {
                let mut value = self.desugar(*value)?;
                let datatype = self.desugar(*datatype)?;
                let kind = match datatype.kind {
                    ir::InstKind::Variable => ir::TypeKind::from(self.render(&span)),
                    _ => self.report(CompilerError::new()
                            .reason("invalid type expression")
                            .span(&datatype.span))?
                };
                value.datatype = ir::InstType::new(datatype.span, kind);
                return Some(value);
            },
            ast::TermKind::BinaryOp { op, left, right } => {
                let callsite = Box::new(ir::Inst::new(op, ir::InstKind::Variable));
                let args = vec![self.desugar(*left)?, self.desugar(*right)?];
                ir::InstKind::FunctionApp { callsite, args }
            },
            ast::TermKind::UnaryOp { op, value } => {
                let callsite = Box::new(ir::Inst::new(op, ir::InstKind::Variable));
                let args = vec![self.desugar(*value)?];
                ir::InstKind::FunctionApp { callsite, args }
            }
        };
        Some(ir::Inst::new(span, kind))
    }
}

/// Desugars an AST into an IR instruction.
pub fn surface_into_core(term : ast::Term, src : &str, issues : &mut IssueTracker) -> Option<ir::Inst> {
    IRBuilder::new(src, issues).desugar(term)
}
