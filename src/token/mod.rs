use super::source_pos::Span;

/// Stores a token and its location in the source file.
#[derive(Debug)]
pub struct Token<'a> {
    pub kind : TokenKind,
    pub span : Span<'a>
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
    While,
    Until,
    Repeat,
    Function,
    Object,
    New,
    Trait,
    Instance,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBox,
    RightBox,
    Dot,
    Comma,
    Colon,
    SemiColon,
    Arrow,
    Assign,
    Identifier(IdentifierKind),
    Literal(LiteralKind)
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