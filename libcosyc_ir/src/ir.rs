use std::fmt;
use libcosyc_diagnostic::source::Span;

/// Represents the possible types of instructions.
#[derive(Debug, PartialEq, Eq, Clone)]
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

impl fmt::Display for TypeKind {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Void => write!(out, "void"),
            Self::Empty => write!(out, "empty"),
            Self::Int8 => write!(out, "int8"),
            Self::Int16 => write!(out, "int16"),
            Self::Int32 => write!(out, "int32"),
            Self::Int64 => write!(out, "int64"),
            Self::UInt8 => write!(out, "uint8"),
            Self::UInt16 => write!(out, "uint16"),
            Self::UInt32 => write!(out, "uint32"),
            Self::UInt64 => write!(out, "uint64"),
            Self::Unknown => write!(out, "<unknown>")
        }
    }
}

impl From<&str> for TypeKind {
    fn from(s : &str) -> Self {
        match s {
            "void" => Self::Void,
            "int8" => Self::Int8,
            "int16" => Self::Int16,
            "int32" => Self::Int32,
            "int64" => Self::Int64,
            "uint8" => Self::UInt8,
            "uint16" => Self::UInt16,
            "uint32" => Self::UInt32,
            "uint64" => Self::UInt64,
            _ => Self::Unknown
        }
    }
}

/// Represents a node for the type of an IR instruction.
#[derive(Debug)]
pub struct InstType {
    pub span : Span,
    pub kind : TypeKind
}

impl InstType {
    /// Creates a new type instance.
    pub fn new(span : Span, kind : TypeKind) -> Self {
        Self { span, kind }
    }
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
    pub datatype : InstType,
    pub kind : InstKind
}

impl Inst {
    /// Creates a new typed instruction.
    pub fn new_typed(span : Span, kind : InstKind, datatype : InstType) -> Self {
        Self { span, datatype, kind }
    }

    /// Creates a new untyped instruction.
    pub fn new(span : Span, kind : InstKind) -> Self {
        let datatype = InstType::new(span.clone(), TypeKind::Unknown);
        Self::new_typed(span, kind, datatype)
    }
}
