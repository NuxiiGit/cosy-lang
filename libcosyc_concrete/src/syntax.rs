use libcosyc_diagnostics::source::Span;

/// Represents a kind of expression.
#[derive(Debug)]
pub enum ExprKind {
    Variable,
    Integral,
    Grouping {
        lparen : bool,
        rparen : bool,
        inner : Option<Box<Expr>>
    },
    Malformed
}

/// Represents expression information.
#[derive(Debug)]
pub struct Expr {
    pub span : Span,
    pub kind : ExprKind
}

