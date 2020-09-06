use libcosyc_diagnostics::span::Span;

/// Represents the different primitive variants.
#[derive(Debug)]
pub enum ValueKind {
    Integral
}

/// Represents a kind of terminal expression.
#[derive(Debug)]
pub enum TerminalKind {
    Variable,
    Value(ValueKind)
}

/// Represents a terminal value
#[derive(Debug)]
pub struct Terminal {
    span : Span,
    kind : TerminalKind
}

/// Represents a kind of expression.
#[derive(Debug)]
pub enum ExprKind {
    Terminal(Terminal)
}

/// Represents expression information
#[derive(Debug)]
pub struct Expr {
    kind : ExprKind
}

