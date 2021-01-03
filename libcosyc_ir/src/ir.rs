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

/// Represents a kind of expression.
#[derive(Debug)]
pub enum InstKind {
    Integral,
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

/// Represents the kinds of primitive types.
#[derive(Debug)]
pub enum Type {
    I8,
    Unknown
}

/// Represents a node for the typed intermediate representation of a program.
#[derive(Debug)]
pub struct Inst {
    pub span : Span,
    pub datatype : Type,
    pub kind : InstKind
}
