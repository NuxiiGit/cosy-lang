#![allow(dead_code)]

/// An enum which describes available token types.
#[derive(Debug)]
pub enum Token<'a> {
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
    Str(&'a str),
    Int(&'a str),
    Ident(&'a str),
    Op(&'a str)
}