/// Represents token types.
#[derive(PartialEq, Eq, Debug)]
pub enum TokenKind {
    Let,
    LeftParen,
    RightParen,
    Plus,
    Minus,
    Integral,
    Hole,
    Identifier,
    RawIdentifier {
        closed : bool
    },
    Comment,
    Whitestuff,
    EoF,
    Unknown
}

impl TokenKind {
    /// Returns whether this token is an identifier.
    pub fn is_identifier(&self) -> bool {
        matches!(self,
                Self::Hole
                | Self::Identifier
                | Self::RawIdentifier { .. })
    }

    /// Returns whether this token indicates a terminal value.
    pub fn is_terminal(&self) -> bool {
        self.is_identifier() || matches!(self, Self::Integral)
    }
}
