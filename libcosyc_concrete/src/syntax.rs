use libcosyc_source::Span;

/// Represents the different primitive variants.
#[derive(Debug)]
pub enum ValueKind {
    Integral
}

/// Represents a kind of expression.
#[derive(Debug)]
pub enum ExprKind {
    Variable,
    Value(ValueKind)
}

/// Represents expression information
#[derive(Debug)]
pub struct Expr {
    pub span : Span,
    pub kind : Option<ExprKind>
}
