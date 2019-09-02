#![allow(dead_code)]

use std::fmt;

/// A struct which stores location information about a `TokenType`.
#[allow(dead_code)]
pub struct Token<'a> {
    flavour : TokenType<'a>,
    row : usize,
    col : usize
}
impl<'a> Token<'a> {
    /// Creates a new instance of `Token`.
    #[allow(dead_code)]
    pub fn new(flavour : TokenType, row : usize, col : usize) -> Token {
        Token {
            flavour : flavour,
            row : row,
            col : col
        }
    }

    /// Returns a reference to the type of this `Token`.
    pub fn flavour(&self) -> &TokenType {
        &self.flavour
    }

    /// Returns the current position as a two value tuple of `(row, column)`.
    pub fn position(&mut self) -> (usize, usize) {
        (self.row, self.col)
    }
}
impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.flavour)
    }
}

/// An enum which stores the type of `Token`.
#[derive(Debug)]
pub enum TokenType<'a> {
    // literals
    String(&'a str),
    Integer(&'a str),
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
    SemiColon
}
impl<'a> fmt::Display for TokenType<'a> {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}