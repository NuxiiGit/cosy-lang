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
            ast::TermKind::Variable =>
                    self.report(CompilerError::unimplemented("IR for variables")
                            .span(&span))?,
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

    /// Assigns the type of an IR instruction.
    /// # Errors
    /// Returns `None` if there was a problem assigning the type.
    pub fn annotate(&mut self, mut value : ir::Inst, ty : ir::Inst) -> Option<ir::Inst> {
        if !matches!(value.datatype, ir::TypeKind::Unknown) {
            self.report(CompilerError::new()
                    .span(&value.span)
                    .reason("this term already has a known type"))?;
        }
        let ty = self.evaluate(ty)?;
        value.datatype = match ty.kind {
            ir::InstKind::Value(kind) => {
                match kind {
                    ir::ValueKind::TypeI8 => ir::TypeKind::I8,
                    ir::ValueKind::TypeType => ir::TypeKind::Type,
                    _ => self.report(CompilerError::new()
                            .span(&ty.span)
                            .reason("invalid type")
                            .note("types cannot be runtime values"))?
                }
            },
            _ => self.report(CompilerError::new()
                    .span(&ty.span)
                    .reason("invalid type annotation")
                    .note("this term must compute to a value"))?
        };
        Some(value)
    }

    /// Evaluates this instruction and produces a new instruction.
    /// # Errors
    /// Returns `None` if the instruction cannot be evaluated at compile-time.
    pub fn evaluate(&mut self, inst : ir::Inst) -> Option<ir::Inst> {
        let span = inst.span;
        let inst = match inst.kind {
            x@ir::InstKind::Value(_) => ir::Inst::new(span, x),
            ir::InstKind::TypeAnno { value, ty } => {
                let value = self.evaluate(*value)?;
                let ty = *ty;
                self.annotate(value, ty)?
            },
            ir::InstKind::BinaryOp { kind : _, left : _, right : _ } =>
                    self.report(CompilerError::unimplemented("compile-time evaluation of binary ops")
                            .span(&span))?,
            ir::InstKind::UnaryOp { kind : _, value : _ } =>
                    self.report(CompilerError::unimplemented("compile-time evaluation of unary ops")
                            .span(&span))?
        };
        Some(inst)
    }

    /// Evaluates constant contexts and produces a new instruction.
    pub fn evaluate_const(&mut self, inst : ir::Inst) -> Option<ir::Inst> {
        let span = inst.span;
        let inst = match inst.kind {
            x@ir::InstKind::Value(_) => ir::Inst::new(span, x),
            ir::InstKind::TypeAnno { value, ty } => {
                let value = self.evaluate_const(*value)?;
                let ty = *ty;
                self.annotate(value, ty)?
            },
            ir::InstKind::BinaryOp { kind, left, right } => {
                let left = Box::new(self.evaluate_const(*left)?);
                let right = Box::new(self.evaluate_const(*right)?);
                let kind = ir::InstKind::BinaryOp { kind, left, right };
                ir::Inst::new(span, kind)
            },
            ir::InstKind::UnaryOp { kind, value } => {
                let value = Box::new(self.evaluate_const(*value)?);
                let kind = ir::InstKind::UnaryOp { kind, value };
                ir::Inst::new(span, kind)
            }
        };
        Some(inst)
    }
}

/// Applies semantic analysis to this AST and returns valid IR.
pub fn generate_ir(ast : ast::Term, src : &str, issues : &mut IssueTracker) -> Option<ir::Inst> {
    let mut man = IRManager::new(src, issues);
    let ir = man.desugar(ast)?;
    let ir = man.evaluate_const(ir)?;
    // TODO evaluate constant terms
    // TODO type infer
    // TODO type check
    Some(ir)
}
