use crate::diagnostics::Span;

use std::fmt;

/// Stores a token and its location in the source file.
pub struct Token<'a> {
    pub kind : TokenKind,
    pub span : Span<'a>
}
impl Token<'_> {
    /// Returns `true` if this token contains any of these prefixes.
    pub fn contains_prefix(&self, prefixes : &[char]) -> bool {
        if let Some(prefix) = self.span.content.chars().next() {
            prefixes.iter().any(|x| *x == prefix)
        } else {
            false
        }
    }
}
impl fmt::Debug for Token<'_> {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "{}", self)
    }
}
impl fmt::Display for Token<'_> {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "{}", self.span.content)
    }
}

/// An enum which describes available token types.
#[derive(PartialEq, Debug, Clone)]
pub enum TokenKind {
    Var,
    Const,
    If,
    Unless,
    Else,
    Then,
    Switch,
    Case,
    Is,
    While,
    Until,
    Repeat,
    For,
    In,
    Function,
    Object,
    New,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBox,
    RightBox,
    Dot,
    Comma,
    Colon,
    ColonColon,
    SemiColon,
    Arrow,
    Assign,
    Dollar,
    Backtick,
    Backslash,
    Identifier(IdentifierKind),
    Literal(LiteralKind),
    EoF,
    Documentation,
    Unknown
}

/// An enum which describes available identifier types.
#[derive(PartialEq, Debug, Clone)]
pub enum IdentifierKind {
    Alphanumeric,
    Operator,
    Empty
}

/// An enum which describes available literal types.
#[derive(PartialEq, Debug, Clone)]
pub enum LiteralKind {
    String,
    Character,
    Integer,
    Real
}