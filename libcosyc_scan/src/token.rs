/// Represents token types.
#[derive(PartialEq, Eq, Debug)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    Pound,
    Colon,
    Plus,
    Minus,
    LeftPipe,
    RightPipe,
    Hole,
    Identifier,
    RawIdentifier {
        closed : bool
    },
    Integral,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    Type,
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
                Self::Integral
                | Self::I8
                | Self::I16
                | Self::I32
                | Self::I64
                | Self::U8
                | Self::U16
                | Self::U32
                | Self::U64
                | Self::Type)
    }
}
