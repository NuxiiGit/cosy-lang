#![allow(dead_code)]

/// A struct which stores location information about a `TokenType`.
pub struct Token<'a> {
    pub flavour : TokenType<'a>,
    pub row : usize,
    pub column : usize
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
    Colon,
    SemiColon
}