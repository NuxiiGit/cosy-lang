pub mod ir;

use libcosyc_diagnostic::{
    error::{
        IssueTracker,
        CompilerError
    },
    source::Span
};
use libcosyc_parse::syntax as ast;

/// Manages the conversion and validation of IR.
pub struct IRManager<'a> {
    src : &'a str,
    issues : &'a mut IssueTracker
}

impl<'a> IRManager<'a> {
    /// Creates a new instance from this issue tracker and source file.
    pub fn new(src : &'a str, issues : &'a mut IssueTracker) -> Self {
        Self { src, issues }
    }

    /// Reports an error to the issue tracker.
    pub fn report<T>(&mut self, error : CompilerError) -> Option<T> {
        self.issues.report_error(error);
        None
    }

    /// Renders this span using the content from the source file.
    pub fn render(&self, span : &Span) -> &'a str {
        span.render(&self.src)
    }

    /// Generates instructions from AST terms.
    pub fn desugar(&mut self, term : ast::Term) -> Option<ir::Inst> {
        let span = term.span;
        let kind = match term.kind {
            ast::TermKind::Variable => unimplemented!(),
            ast::TermKind::Const(kind) => {
                let kind = match kind {
                    ast::ConstKind::Integral => ir::ValueKind::Integral,
                    ast::ConstKind::I8 => ir::ValueKind::TypeI8,
                    ast::ConstKind::Type => ir::ValueKind::TypeType
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
                    ast::BinaryOpKind::Custom(op) => self.report(CompilerError::bug()
                            .span(&span)
                            .reason("infix function application is not currently supported")
                            .note(format!("consider refactoring this to `{}({}, {})`",
                                    self.render(&op.span), self.render(&left.span), self.render(&right.span))))?
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
            },
        };
        Some(ir::Inst::new(span, kind))
    }
}

/// Applies semantic analysis to this AST and returns valid IR.
pub fn generate_ir(ast : ast::Term, src : &str, issues : &mut IssueTracker) -> Option<ir::Inst> {
    let mut man = IRManager::new(src, issues);
    let ir = man.desugar(ast)?;
    // TODO evaluate constant terms
    // TODO type infer
    // TODO type check
    Some(ir)
}
