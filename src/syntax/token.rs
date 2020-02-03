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
    Symbol(SymbolKind),
    Identifier,
    Operator(OperatorKind),
    Literal(LiteralKind),
    EoF,
    Epsilon,
    Directive,
    Documentation,
    Unknown
}
impl TokenKind {
    /// Returns `true` of the token is a keyword.
    pub fn is_keyword(&self) -> bool {
        if let TokenKind::Keyword(..) = self { true } else { false }
    }

    /// Returns `true` of the token is an identifier.
    pub fn is_identifier(&self) -> bool {
        if let TokenKind::Identifier = self { true } else { false }
    }

    /// Returns `true` of the token is an operator.
    pub fn is_operator(&self) -> bool {
        if let TokenKind::Operator(..) = self { true } else { false }
    }

    /// Returns `true` of the token is a literal.
    pub fn is_literal(&self) -> bool {
        if let TokenKind::Literal(..) = self { true } else { false }
    }

    /// Returns `true` of the token is the end of the file.
    pub fn is_eof(&self) -> bool {
        if let TokenKind::EoF = self { true } else { false }
    }

    /// Returns `true` of the token is a compiler directive.
    pub fn is_directive(&self) -> bool {
        if let TokenKind::Directive = self { true } else { false }
    }

    /// Returns `true` of the token is documentation.
    pub fn is_documentation(&self) -> bool {
        if let TokenKind::Documentation = self { true } else { false }
    }

    /// Returns `true` of the token is unknown.
    pub fn is_unknown(&self) -> bool {
        if let TokenKind::Unknown = self { true } else { false }
    }
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
    New
}

/// An enum which describes available symbol types.
#[derive(PartialEq, Debug, Clone)]
pub enum SymbolKind {
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
    Dollar,
    Backtick,
    Assignment,
    Hashtag,
    Address
}

/// An enum which describes available operator types.
#[derive(PartialEq, Debug, Clone)]
pub enum OperatorKind {
    Bar,
    Caret,
    Ampersand,
    Bang,
    Equals,
    LessThan,
    GreaterThan,
    Plus,
    Minus,
    Asterisk,
    ForwardSlash,
    BackSlash,
    Percent,
    Other
}

/// An enum which describes available literal types.
#[derive(PartialEq, Debug, Clone)]
pub enum LiteralKind {
    String,
    Character,
    Integer,
    Real
}