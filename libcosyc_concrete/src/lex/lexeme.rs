/// Represents literal types.
#[derive(PartialEq, Debug, Clone)]
pub enum LiteralKind {
    Integral
}

/// Represents identifier types.
#[derive(PartialEq, Debug, Clone)]
pub enum IdentifierKind {
    Graphic,
    Multiplication,
    Addition,
    Comparison,
    And,
    Or,
    Equality,
    Application,
    Other
}

/// Represents token types.
#[derive(PartialEq, Debug, Clone)]
pub enum TokenKind {
    Literal(LiteralKind),
    Identifier(IdentifierKind),
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    SemiColon,
    Let,
    Assign,
    EoF,
    Unknown
}
impl TokenKind {
    /// Returns `true` if the token is a literal value.
    pub fn is_literal(&self) -> bool {
        matches!(self, Self::Literal(..))
    }

    /// Returns `true` if the token is an identifier.
    pub fn is_identifier(&self) -> bool {
        matches!(self, Self::Identifier(..))
    }

    /// Returns `true` if the token is an alphabetic identifier.
    pub fn is_graphic(&self) -> bool {
        matches!(self, Self::Identifier(IdentifierKind::Graphic))
    }

    /// Returns whether this token is a valid terminal value.
    pub fn is_terminal(&self) -> bool {
        self.is_literal() || self.is_identifier()
    }

    /// Returns `true` if the token is an operator identifier.
    pub fn is_operator(&self) -> bool {
        self.is_identifier() && !self.is_graphic()
    }

    /// Returns `true` if the token is the end of the file.
    pub fn is_eof(&self) -> bool {
        matches!(self, Self::EoF)
    }
}
