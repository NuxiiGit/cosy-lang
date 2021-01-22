use libcosyc_diagnostic::source::Span;

/// Represents the different kinds of binary operation.
#[derive(Debug)]
pub enum BinaryOpKind {
    Add,
    Subtract,
    Custom(Box<Term>)
}

/// Represents the different kinds of binary operation.
#[derive(Debug)]
pub enum UnaryOpKind {
    Negate
}

/// Represents the different kinds of constants.
#[derive(Debug)]
pub enum ConstKind {
    Integral,
    I8,
    TypeUniverse(usize)
}

/// Represents a kind of term.
#[derive(Debug)]
pub enum TermKind {
    Variable,
    Const(ConstKind),
    TypeAnno {
        value : Box<Term>,
        ty : Box<Term>
    },
    BinaryOp {
        kind : BinaryOpKind,
        left : Box<Term>,
        right : Box<Term>
    },
    UnaryOp {
        kind : UnaryOpKind,
        value : Box<Term>
    }
}

/// Represents AST terms.
#[derive(Debug)]
pub struct Term {
    pub span : Span,
    pub kind : TermKind
}
