use crate::syntax::token::*;

/// A recursive enum which stores expression information.
#[derive(Debug)]
pub enum Expr<'a> {
    Literal(Token<'a>),
    Variable(Token<'a>),
    Member(Box<Expr<'a>>, Token<'a>),
    Call(Box<Expr<'a>>, Box<Expr<'a>>)
}