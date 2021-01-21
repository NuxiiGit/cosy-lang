use libcosyc_diagnostic::source::Span;
use std::fmt;

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
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeKind {
    I8,
    Type,
    Unknown
}

impl fmt::Display for TypeKind {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::I8 => write!(out, "i8"),
            Self::Type => write!(out, "type"),
            Self::Unknown => write!(out, "<unknown>")
        }
    }
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
    /// Creates a new typed instruction.
    pub fn new_typed(span : Span, kind : InstKind, datatype : TypeKind) -> Self {
        Self { span, datatype, kind }
    }

    /// Creates a new untyped instruction.
    pub fn new(span : Span, kind : InstKind) -> Self {
        Self::new_typed(span, kind, TypeKind::Unknown)
    }
}
