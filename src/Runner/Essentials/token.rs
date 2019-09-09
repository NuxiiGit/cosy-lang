#![allow(dead_code)]

use std::fmt;

/// A struct which stores location information about a `TokenType`.
pub struct Token<'a> {
    pub flavour : TokenType<'a>,
    pub row : usize,
    pub column : usize
}
impl<'a> fmt::Debug for Token<'a> {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.flavour)
    }
}

/// An enum which stores the type of `Token`.
#[derive(Debug)]
pub enum TokenType<'a> {
    // literals
    Literal(&'static str, &'a str),
    // Keywords
    Identifier(&'a str),
    Var,
    If,
    IfNot,
    Else,
    // Symbols
    Operator(&'a str),
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Colon,
    SemiColon
}