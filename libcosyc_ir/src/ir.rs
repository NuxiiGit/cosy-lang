use libcosyc_diagnostic::source::Span;

/// Represents the different kinds of binary operation.
#[derive(Debug)]
pub enum BinaryOpKind {
    Add,
    Subtract
}

/// Represents the different kinds of binary operation.
#[derive(Debug)]
pub enum UnaryOpKind {
    Negate
}

/// Represents the different kinds of constants.
#[derive(Debug)]
pub enum ConstKind {
    Integral
}

/// Represents the different kinds of primitive types.
#[derive(Debug)]
pub enum PrimitiveKind {
    I8,
    Type
}

/// Represents a kind of expression.
#[derive(Debug)]
pub enum InstKind {
    Const(ConstKind),
    Primitive(PrimitiveKind),
    BinaryOp {
        kind : BinaryOpKind,
        linst : Box<Inst>,
        rinst : Box<Inst>
    },
    UnaryOp {
        kind : UnaryOpKind,
        inner : Box<Inst>
    }
}

/// Represents a node for the typed intermediate representation of a program.
#[derive(Debug)]
pub struct Inst {
    pub span : Span,
    pub datatype : Option<Box<Inst>>,
    pub kind : InstKind
}

impl Inst {
    /// Creates a new untyped instruction.
    pub fn new_typed(span : Span, kind : InstKind, datatype : Inst) -> Self {
        let datatype = Some(Box::new(datatype));
        Self { span, datatype, kind }
    }

    /// Creates a new untyped instruction.
    pub fn new_untyped(span : Span, kind : InstKind) -> Self {
        let datatype = None;
        Self { span, datatype, kind }
    }
}
