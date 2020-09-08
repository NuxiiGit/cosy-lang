use libcosyc_source::Span;

/// Represents a kind of terminal value.
#[derive(Debug)]
pub enum TerminalKind {
    Variable,
    Integral
}

/// Represents a kind of expression.
#[derive(Debug)]
pub enum ExprKind {
    Terminal(Option<TerminalKind>)
}

/// Represents expression information.
#[derive(Debug)]
pub struct Expr {
    pub span : Span,
    pub kind : ExprKind
}
