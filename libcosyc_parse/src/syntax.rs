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

/// Represents a kind of term.
#[derive(Debug)]
pub enum TermKind {
    Variable,
    Integral,
    Primitive,
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
    },
    Call {
        intrinsic : bool,
        callsite : Box<Term>,
        params : Vec<Term>
    }
}

/// Represents AST terms.
#[derive(Debug)]
pub struct Term {
    pub span : Span,
    pub kind : TermKind
}

