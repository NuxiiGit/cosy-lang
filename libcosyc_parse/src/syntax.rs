use libcosyc_diagnostic::source::Span;

/// Represents the different kinds of binary operation.
#[derive(Debug)]
pub enum BinaryOpKind {
    Addition,
    Subtraction,
    Custom
}

/// Represents the different kinds of binary operation.
#[derive(Debug)]
pub enum UnaryOpKind {
    Negate
}

/// Represents a kind of expression.
#[derive(Debug)]
pub enum ExprKind {
    Variable,
    Integral,
    BinaryOp {
        kind : BinaryOpKind,
        left : Box<Expr>,
        right : Box<Expr>
    },
    UnaryOp {
        kind : UnaryOpKind,
        inner : Box<Expr>
    },
    Call {
        intrinsic : bool,
        params : Vec<Expr>
    }
}

/// Represents expression information.
#[derive(Debug)]
pub struct Expr {
    pub span : Span,
    pub kind : ExprKind
}

