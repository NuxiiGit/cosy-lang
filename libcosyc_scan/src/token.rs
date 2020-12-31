/// Represents literal types.
#[derive(PartialEq, Eq, Debug)]
pub enum LiteralKind {
    Integral
}

/// Represents different keyword types.
#[derive(PartialEq, Eq, Debug)]
pub enum GraphicKind {
    Let,
    Hole,
    Other
}

/// Represents identifier types.
#[derive(PartialEq, Eq, Debug)]
pub enum IdentifierKind {
    Graphic(GraphicKind),
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

impl TokenKind {
    /// Returns whether this token is a literal.
    pub fn is_literal(&self) -> bool {
        matches!(self, Self::Literal(..))
    }

    /// Returns whether this token is an identifier.
    pub fn is_identifier(&self) -> bool {
        matches!(self, Self::Identifier(..))
    }
}
