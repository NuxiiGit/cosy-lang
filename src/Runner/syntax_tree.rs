#![allow(dead_code)]

/// An enum which stores the root of the syntax tree.
#[derive(Debug)]
pub enum SyntaxTree<'a> {
    Expression(Box<Expr<'a>>)
}

/// A recursive enum which stores expression information.
#[derive(Debug)]
pub enum Expr<'a> {
    // literals
    Integer(&'a str),
    String(&'a str),
    Variable(&'a str),
    // operators
    Unary(&'a str, Box<Expr<'a>>),
    Binary(&'a str, Box<Expr<'a>>, Box<Expr<'a>>)
}