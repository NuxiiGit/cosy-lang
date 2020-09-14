use libcosyc_diagnostics::source::Span;

/// Represents a kind of expression.
#[derive(Debug)]
pub enum ExprKind {
    Variable,
    Integral,
    Empty
}

/// Represents expression information.
#[derive(Debug)]
pub struct Expr {
    pub span : Span,
    pub kind : ExprKind
}

