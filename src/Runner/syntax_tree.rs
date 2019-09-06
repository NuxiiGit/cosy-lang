#![allow(dead_code)]

use super::token::*;

/// An enum which stores the root of the syntax tree.
pub enum SyntaxTree<'a> {
    Expression(Expr<'a>)
}

/// A recursive enum which stores expression information.
pub enum Expr<'a> {
    Terminal(Token<'a>),
    Unary(Token<'a>, Box<Expr<'a>>),
    Binary(Token<'a>, Box<Expr<'a>>, Box<Expr<'a>>),
}