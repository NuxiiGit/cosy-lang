use libcosyc_diagnostic::source::Span;

/// Represents the different kinds of binary operation.
#[derive(Debug)]
pub enum BinaryOpKind {
    Add,
    Subtract,
    Custom(Box<Expr>)
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
    Primitive,
    TypeAnno {
        vexpr : Box<Expr>,
        texpr : Box<Expr>
    },
    BinaryOp {
        kind : BinaryOpKind,
        lexpr : Box<Expr>,
        rexpr : Box<Expr>
    },
    UnaryOp {
        kind : UnaryOpKind,
        inner : Box<Expr>
    },
    Call {
        intrinsic : bool,
        callsite : Box<Expr>,
        params : Vec<Expr>
    }
}

/// Represents expression information.
#[derive(Debug)]
pub struct Expr {
    pub span : Span,
    pub kind : ExprKind
}

