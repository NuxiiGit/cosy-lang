#![allow(dead_code)]

use super::token::*;

/// An enum which stores the root of the syntax tree.
#[derive(Debug)]
pub enum SyntaxTree<'a> {
    Expression(Expr<'a>)
}

/// A recursive enum which stores expression information.
#[derive(Debug)]
pub enum Expr<'a> {
    Literal(Token<'a>),
    Field(Token<'a>),
    Unary(Token<'a>, Box<Expr<'a>>),
    Binary(Token<'a>, Box<Expr<'a>>, Box<Expr<'a>>),
}