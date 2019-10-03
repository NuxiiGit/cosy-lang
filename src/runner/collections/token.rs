#![allow(dead_code)]

use std::fmt;

/// A struct which stores token location data.
#[derive(Debug)]
pub struct Token<'a> {
    pub flavour : TokenType<'a>,
    pub row : usize,
    pub column : usize
}
impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.flavour)
    }
}

/// An enum which stores the type of `Token`.
#[derive(Debug)]
pub enum TokenType<'a> {
    Var,
    If,
    IfNot,
    Else,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Colon,
    SemiColon,
    String(&'a str),
    Integer(&'a str),
    Identifier(&'a str),
    Operator(&'a str),
}