use super::span::Span;

/// Stores a token and its location in the source file.
#[derive(Debug, Clone)]
pub struct Token {
    pub kind : TokenKind,
    pub span : Span
}

/// An enum which describes available token types.
#[derive(PartialEq, Debug, Clone)]
pub enum TokenKind {
    Keyword(KeywordKind),
    Identifier,
    Operator(OperatorKind),
    Literal(LiteralKind),
    EoF,
    Directive,
    Documentation,
    Unknown
}

/// An enum which describes available keyword types.
#[derive(PartialEq, Debug, Clone)]
pub enum KeywordKind {
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
    Backslash
}

/// An enum which describes available operator types.
#[derive(PartialEq, Debug, Clone)]
pub enum OperatorKind {
    Custom
}

/// An enum which describes available literal types.
#[derive(PartialEq, Debug, Clone)]
pub enum LiteralKind {
    String,
    Character,
    Integer,
    Real
}