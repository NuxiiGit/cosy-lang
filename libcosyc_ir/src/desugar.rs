use crate::ir;
use libcosyc_diagnostic::{
    error::{ CompilerError, IssueTracker, Failable },
    source::Renderable
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
            ast::TermKind::Variable =>
                    self.report(CompilerError::unimplemented("IR for variables")
                            .span(&span))?,
            ast::TermKind::Const(kind) => {
                let kind = match kind {
                    ast::ConstKind::Integral => {
                        if let Ok(n) = self.render(&span).parse::<u64>() {
                            ir::ValueKind::U64(n)
                        } else {
                            self.report(CompilerError::new()
                                    .span(&span)
                                    .reason("unable to parse integer literal"))?
                        }
                    },
                    ast::ConstKind::I8 => ir::ValueKind::TypeI8,
                    ast::ConstKind::I16 => ir::ValueKind::TypeI16,
                    ast::ConstKind::I32 => ir::ValueKind::TypeI32,
                    ast::ConstKind::I64 => ir::ValueKind::TypeI64,
                    ast::ConstKind::U8 => ir::ValueKind::TypeU8,
                    ast::ConstKind::U16 => ir::ValueKind::TypeU16,
                    ast::ConstKind::U32 => ir::ValueKind::TypeU32,
                    ast::ConstKind::U64 => ir::ValueKind::TypeU64,
                    ast::ConstKind::TypeUniverse(n) => ir::ValueKind::TypeUniverse(n)
                };
                ir::InstKind::Value(kind)
            },
            ast::TermKind::TypeAnno { value, ty } => {
                let value = Box::new(self.desugar(*value)?);
                let ty = Box::new(self.desugar(*ty)?);
                ir::InstKind::TypeAnno { value, ty }
            },
            ast::TermKind::BinaryOp { kind, left, right } => {
                let kind = match kind {
                    ast::BinaryOpKind::Add => ir::BinaryOpKind::Add,
                    ast::BinaryOpKind::Subtract => ir::BinaryOpKind::Subtract,
                    ast::BinaryOpKind::Custom(_) =>
                            self.report(CompilerError::unimplemented("infix function application")
                                    .span(&span))?
                };
                let left = Box::new(self.desugar(*left)?);
                let right = Box::new(self.desugar(*right)?);
                ir::InstKind::BinaryOp { kind, left, right }
            },
            ast::TermKind::UnaryOp { kind, value } => {
                let kind = match kind {
                    ast::UnaryOpKind::Negate => ir::UnaryOpKind::Negate,
                };
                let value = Box::new(self.desugar(*value)?);
                ir::InstKind::UnaryOp { kind, value }
            }
        };
        Some(ir::Inst::new(span, kind))
    }
}

/// Desugars an AST into an IR instruction.
pub fn surface_into_core(term : ast::Term, src : &str, issues : &mut IssueTracker) -> Option<ir::Inst> {
    IRBuilder::new(src, issues).desugar(term)
}
