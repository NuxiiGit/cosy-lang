use libcosyc_diagnostics::source::Span;

/// Represents a program.
pub type Program = Vec<Stmt>;

/// Represents a kind of statement.
#[derive(Debug)]
pub enum StmtKind {
    Expr {
        inner : Box<Expr>
    },
    NoOp
}

/// Represents statement information.
#[derive(Debug)]
pub struct Stmt {
    pub span : Span,
    pub kind : StmtKind,
    pub terminated : bool
}

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
    Block {
        lbrace : bool,
        rbrace : bool,
        body : Program
    },
    Malformed
}

/// Represents expression information.
#[derive(Debug)]
pub struct Expr {
    pub span : Span,
    pub kind : ExprKind
}

