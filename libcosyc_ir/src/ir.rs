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
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    TypeI8,
    TypeI16,
    TypeI32,
    TypeI64,
    TypeU8,
    TypeU16,
    TypeU32,
    TypeU64,
    TypeUniverse(usize)
}

impl ValueKind {
    /// Returns whether this value is runtime-known.
    pub fn is_runtime_known(&self) -> bool {
        matches!(self,
                Self::I8(_)
                | Self::I16(_)
                | Self::I32(_)
                | Self::I64(_)
                | Self::U8(_)
                | Self::U16(_)
                | Self::U32(_)
                | Self::U64(_))
    }
}

/// Represents the different kinds of types.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeKind {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    TypeUniverse(usize),
    Unknown
}

impl fmt::Display for TypeKind {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::I8 => write!(out, "i8"),
            Self::I16 => write!(out, "i16"),
            Self::I32 => write!(out, "i32"),
            Self::I64 => write!(out, "i64"),
            Self::U8 => write!(out, "u8"),
            Self::U16 => write!(out, "u16"),
            Self::U32 => write!(out, "u32"),
            Self::U64 => write!(out, "u64"),
            Self::TypeUniverse(n) => {
                write!(out, "type")?;
                if *n > 0 {
                    write!(out, "#{}", *n + 1)?;
                }
                Ok(())
            },
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

/// Infers the types of trivial values.
pub fn infer_value_type(value : &ValueKind) -> TypeKind {
    match value {
        ValueKind::I8(_) => TypeKind::I8,
        ValueKind::I16(_) => TypeKind::I16,
        ValueKind::I32(_) => TypeKind::I32,
        ValueKind::I64(_) => TypeKind::I64,
        ValueKind::U8(_) => TypeKind::U8,
        ValueKind::U16(_) => TypeKind::U16,
        ValueKind::U32(_) => TypeKind::U32,
        ValueKind::U64(_) => TypeKind::U64,
        ValueKind::TypeI8
                | ValueKind::TypeI16
                | ValueKind::TypeI32
                | ValueKind::TypeI64
                | ValueKind::TypeU8
                | ValueKind::TypeU16
                | ValueKind::TypeU32
                | ValueKind::TypeU64 => TypeKind::TypeUniverse(0),
        ValueKind::TypeUniverse(n) => TypeKind::TypeUniverse(*n + 1)
    }
}

/// Converts a type value into a concrete type.
pub fn value_to_type(value : &ValueKind) -> Option<TypeKind> {
    let ty = match value {
        ValueKind::I8(_)
                | ValueKind::I16(_)
                | ValueKind::I32(_)
                | ValueKind::I64(_)
                | ValueKind::U8(_)
                | ValueKind::U16(_)
                | ValueKind::U32(_)
                | ValueKind::U64(_) => return None,
        ValueKind::TypeI8 => TypeKind::I8,
        ValueKind::TypeI16 => TypeKind::I16,
        ValueKind::TypeI32 => TypeKind::I32,
        ValueKind::TypeI64 => TypeKind::I64,
        ValueKind::TypeU8 => TypeKind::U8,
        ValueKind::TypeU16 => TypeKind::U16,
        ValueKind::TypeU32 => TypeKind::U32,
        ValueKind::TypeU64 => TypeKind::U64,
        ValueKind::TypeUniverse(n) => TypeKind::TypeUniverse(*n)
    };
    Some(ty)
}
