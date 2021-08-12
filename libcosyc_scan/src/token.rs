/// Represents token types.
#[derive(PartialEq, Eq, Debug)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBox,
    RightBox,
    LeftBrace,
    RightBrace,
    Colon,
    Hole,
    Identifier,
    RawIdentifier {
        closed : bool
    },
    Operator {
        precedence : u8
    },
    Integral,
    Let,
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
        self.is_identifier() || matches!(self,
                Self::Integral)
    }
}
