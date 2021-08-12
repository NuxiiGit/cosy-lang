use libcosyc_diagnostic::source::Span;

/// Represents the possible types of instructions.
#[derive(Debug)]
pub enum TypeKind {
    Void,
    Empty,
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Unknown
}

/// Represents a kind of expression.
#[derive(Debug)]
pub enum InstKind {
    Variable,
    Integral {
        radix : u8
    },
    FunctionApp {
        callsite : Box<Inst>,
        args : Vec<Inst>
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
