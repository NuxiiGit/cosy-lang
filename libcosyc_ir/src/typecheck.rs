use crate::ir;
use libcosyc_diagnostic::{
    error::{ CompilerError, IssueTracker, Failable },
    source::Renderable
};

macro_rules! integral_types {
    () => {
        vec![ir::TypeKind::I8,
                ir::TypeKind::I16,
                ir::TypeKind::I32,
                ir::TypeKind::I64,
                ir::TypeKind::U8,
                ir::TypeKind::U16,
                ir::TypeKind::U32,
                ir::TypeKind::U64]
    }
}

/// Manages the conversion and validation of IR.
pub struct Typechecker<'a> {
    src : &'a str,
    issues : &'a mut IssueTracker
}

impl Failable for Typechecker<'_> {
    fn issues(&mut self) -> &mut IssueTracker {
        self.issues
    }
}

impl Renderable for Typechecker<'_> {
    fn src(&self) -> &str {
        self.src
    }
}

impl<'a> Typechecker<'a> {
    /// Creates a new instance from this issue tracker and source file.
    pub fn new(src : &'a str, issues : &'a mut IssueTracker) -> Self {
        Self { src, issues }
    }

    /// Asserts whether this instruction has one of the following types.
    pub fn expect_type(&mut self, inst : &ir::Inst, expect : &[ir::TypeKind]) -> Option<()> {
        let span = &inst.span;
        let datatype = &inst.datatype;
        for ty_kind in expect {
            if datatype == ty_kind {
                return Some(());
            }
        }
        let mut types = String::new();
        let count = expect.len();
        for (i, ty_kind) in expect.iter().enumerate() {
            if i != 0 {
                types.push_str(if i + 1 == count { " or" } else { "," })
            }
            types.push_str(" `");
            types.push_str(&ty_kind.to_string());
            types.push_str("`");
        }
        let mut err = CompilerError::new()
                .span(&span)
                .reason(format!("expected a value of type{} (got `{}`)", types, datatype));
        if matches!(datatype, ir::TypeKind::Unknown) {
            err = err.note("consider adding a type annotation");
        }
        self.report(err)
    }

    /// Asserts whether these two terms have equivalent types.
    pub fn expect_equal_types(&mut self, a : &ir::Inst, b : &ir::Inst) -> Option<()> {
        let mut ty_a = &a.datatype;
        let mut ty_b = &b.datatype;
        if ty_a == ty_b {
            return Some(());
        }
        if matches!(ty_a, ir::TypeKind::Unknown) {
            let tmp = ty_a;
            ty_a = ty_b;
            ty_b = tmp;
        }
        let mut err = CompilerError::new()
                .span(&b.span)
                .reason(format!("expected a value of type `{}` (got `{}`)", ty_a, ty_b));
        if matches!(ty_a, ir::TypeKind::Unknown) ||
                matches!(ty_b, ir::TypeKind::Unknown) {
            err = err.note("consider adding a type annotation");
        }
        self.report(err)
    }

    /// Assigns the type of an IR instruction.
    /// # Errors
    /// Returns `None` if there was a problem assigning the type.
    pub fn annotate(&mut self, mut value : ir::Inst, ty : ir::Inst) -> Option<ir::Inst> {
        let ty = self.evaluate(ty)?;
        let datatype = match ty.kind {
            ir::InstKind::Value(kind) => {
                if let Some(datatype) = ir::value_to_type(&kind) {
                    datatype
                } else {
                    self.report(CompilerError::new()
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
        if !matches!(value.datatype, ir::TypeKind::Unknown) {
            self.expect_type(&value, &[datatype.clone()])?;
        }
        value.datatype = datatype;
        Some(value)
    }

    /// Evaluates this instruction and produces a new instruction.
    /// # Errors
    /// Returns `None` if the instruction cannot be evaluated at compile-time.
    pub fn evaluate(&mut self, inst : ir::Inst) -> Option<ir::Inst> {
        let span = inst.span;
        let inst = match inst.kind {
            ir::InstKind::Value(kind) => {
                let datatype = ir::infer_value_type(&kind);
                ir::Inst::new_typed(span, ir::InstKind::Value(kind), datatype)
            },
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
            ir::InstKind::Value(kind) => {
                if !kind.is_runtime_known() {
                    self.report(CompilerError::new()
                            .span(&span)
                            .reason("values of this type cannot be used at runtime"))?
                }
                let datatype = ir::infer_value_type(&kind);
                ir::Inst::new_typed(span, ir::InstKind::Value(kind), datatype)
            },
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

    /// Performs type checking on this instruction and returns whether it is well-typed.
    /// # Errors
    /// Returns `None` if the instruction is not well-typed.
    pub fn typecheck(&mut self, inst : &ir::Inst) -> Option<()> {
        let span = &inst.span;
        match &inst.kind {
            ir::InstKind::Value(kind) => {
                let datatype = ir::infer_value_type(&kind);
                self.expect_type(inst, &[datatype])?;
            },
            ir::InstKind::TypeAnno { .. } =>
                    self.report(CompilerError::unreachable("type ascriptions")
                            .span(&span))?,
            ir::InstKind::BinaryOp { kind, left, right } => {
                let expect = match kind {
                    ir::BinaryOpKind::Add
                            | ir::BinaryOpKind::Subtract => integral_types!()
                };
                self.expect_type(inst, &expect)?;
                self.typecheck(left)?;
                self.typecheck(right)?;
                self.expect_equal_types(inst, left)?;
                self.expect_equal_types(left, right)?;
            },
            ir::InstKind::UnaryOp { kind, value } => {
                let expect = match kind {
                    ir::UnaryOpKind::Negate => integral_types!()
                };
                self.expect_type(inst, &expect)?;
                self.typecheck(value)?;
                self.expect_equal_types(inst, value)?;
            }
        }
        Some(())
    }
}

/// Performs compile-time evaluation and type checking on this IR. Returns validated IR.
pub fn check(inst : ir::Inst, src : &str, issues : &mut IssueTracker) -> Option<ir::Inst> {
    let mut man = Typechecker::new(src, issues);
    let inst = man.evaluate_const(inst)?;
    man.typecheck(&inst)?; // TODO type infer
    Some(inst)
}
