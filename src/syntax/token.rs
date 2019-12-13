use crate::diagnostics::Span;

use std::fmt;

/// Stores a token and its location in the source file.
pub struct Token<'a> {
    pub kind : TokenKind,
    pub span : Span<'a>
}
impl fmt::Debug for Token<'_> {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "{:?} [{}]", self.kind, self.span.content)
    }
}

/// An enum which describes available token types.
#[derive(PartialEq, Debug)]
pub enum TokenKind {
    Empty,
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
    Documentation,
    Identifier(IdentifierKind),
    Literal(LiteralKind),
    EoF,
    Unknown
}

/// An enum which describes available identifier types.
#[derive(PartialEq, Debug)]
pub enum IdentifierKind {
    Alphanumeric,
    Operator,
    Literal
}

/// An enum which describes available literal types.
#[derive(PartialEq, Debug)]
pub enum LiteralKind {
    String,
    Character,
    Integer,
    Real
}