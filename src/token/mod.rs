use super::source_pos::Span;

/// Stores a token and its location in the source file.
#[derive(Debug)]
pub struct Token<'a> {
    kind : TokenKind,
    span : Span<'a>
}

/// An enum which describes available token types.
#[derive(PartialEq, Debug)]
pub enum TokenKind {
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
    Identifier,
    Literal(LiteralKind),
    Eof
}

/// An enum which describes available literal types.
#[derive(PartialEq, Debug)]
pub enum LiteralKind {
    String,
    Character,
    Integer,
    Real
}