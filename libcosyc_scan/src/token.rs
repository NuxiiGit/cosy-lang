/// Represents literal types.
#[derive(PartialEq, Eq, Debug)]
pub enum LiteralKind {
    Integral
}

/// Represents identifier types.
#[derive(PartialEq, Eq, Debug)]
pub enum IdentifierKind {
    Let,
    Hole,
    Graphic,
    Raw {
        closed : bool
    }
}

/// Represents token types.
#[derive(PartialEq, Eq, Debug)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    Plus,
    Literal(LiteralKind),
    Identifier(IdentifierKind),
    Comment,
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
