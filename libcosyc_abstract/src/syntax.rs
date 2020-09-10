use libcosyc_diagnostics::Span;

/// Represents a kind of expression.
#[derive(Debug)]
pub enum ExprKind {
    Variable,
    Integral
}

/// Represents expression information.
#[derive(Debug)]
pub struct Expr {
    pub span : Span,
    pub kind : ExprKind
}
