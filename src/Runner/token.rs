#![allow(dead_code)]

/// A struct which stores location information about a `TokenType`.
pub struct Token<'a> {
    flavour : TokenType<'a>,
    row : usize,
    col : usize
}
impl<'a> Token<'a> {
    /// Creates a new instance of `Token`.
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