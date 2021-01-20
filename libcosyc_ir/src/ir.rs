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

/// Represents the different kinds of value.
#[derive(Debug)]
pub enum ValueKind {
    Integral,
    TypeI8,
    TypeType
}

/// Represents the different kinds of types.
#[derive(Debug, PartialEq, Eq)]
pub enum TypeKind {
    I8,
    Type,
    Unknown
}

/// Represents a kind of expression.
#[derive(Debug)]
pub enum InstKind {
    Value(ValueKind),
    TypeAnno {
        value : Box<Inst>,
        ty : Box<Inst>
    },
    BinaryOp {
        kind : BinaryOpKind,
        left : Box<Inst>,
        right : Box<Inst>
    },
    UnaryOp {
        kind : UnaryOpKind,
        value : Box<Inst>
    }
}

/// Represents a node for the typed intermediate representation of a program.
#[derive(Debug)]
pub struct Inst {
    pub span : Span,
    pub datatype : TypeKind,
    pub kind : InstKind
}

impl Inst {
    /// Creates a new untyped instruction.
    pub fn new(span : Span, kind : InstKind) -> Self {
        let datatype = TypeKind::Unknown;
        Self { span, datatype, kind }
    }
}
