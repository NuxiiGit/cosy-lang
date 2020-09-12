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

/// Represents a kind of statement.
#[derive(Debug)]
pub enum StmtKind {
    Expr {
        inner : Box<Expr>
    }
}

/// Represents statement information.
#[derive(Debug)]
pub struct Stmt {
    pub span : Span,
    pub kind : StmtKind
}
