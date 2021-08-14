use std::fmt;
use libcosyc_diagnostic::source::Span;

/// Represents the possible types of instructions.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TypeKind {
    /// A type that has not been evaluated yet.
    Variable,
    /// A type that should be inferred by the compiler.
    Infer,
    /// The type of non-terminating programs.
    Void,
    /// The type of statements and boring functions.
    Empty,
    /// Signed integers.
    Int(u8),
    /// Unsigned integers.
    UInt(u8),
}

impl fmt::Display for TypeKind {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Variable => write!(out, "<variable>"),
            Self::Infer => write!(out, "<infer>"),
            Self::Void => write!(out, "void"),
            Self::Empty => write!(out, "empty"),
            Self::Int(n) => write!(out, "int{}", n),
            Self::UInt(n) => write!(out, "uint{}", n),
        }
    }
}

impl TypeKind {
    /// Attempts to create a primitive type from an identifier.
    pub fn from_name(str : &str) -> Option<Self> {
        let ty = match str {
            "void" => Self::Void,
            "int8" => Self::Int(8),
            "int16" => Self::Int(16),
            "int32" => Self::Int(32),
            "int64" => Self::Int(64),
            "uint8" => Self::UInt(8),
            "uint16" => Self::UInt(16),
            "uint32" => Self::UInt(32),
            "uint64" => Self::UInt(64),
            _ => return None
        };
        Some(ty)
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
        let datatype = InstType::new(span.clone(), TypeKind::Infer);
        Self::new_typed(span, kind, datatype)
    }
}
