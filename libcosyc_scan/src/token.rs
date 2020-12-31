/// Represents literal types.
#[derive(PartialEq, Eq, Debug)]
pub enum LiteralKind {
    Integral
}

/// Represents different keyword types.
#[derive(PartialEq, Eq, Debug)]
pub enum KeywordKind {
    Let
}

/// Represents identifier types.
#[derive(PartialEq, Eq, Debug)]
pub enum IdentifierKind {
    Contextual(KeywordKind),
    Graphic,
    Addition,
    Other
}

/// Represents token types.
#[derive(PartialEq, Eq, Debug)]
pub enum TokenKind {
    Literal(LiteralKind),
    Identifier(IdentifierKind),
    LeftParen,
    RightParen,
    EoF,
    Unknown
}
