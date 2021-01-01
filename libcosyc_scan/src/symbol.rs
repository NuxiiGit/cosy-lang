/// Represents various kinds of character types.
#[derive(PartialEq, Eq, Debug)]
pub enum SymbolKind {
    Whitestuff,
    Digit,
    Graphic,
    Underscore,
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
    Dollar,
    Backtick,
    Hashtag,
    Address,
    DoubleQuote,
    SingleQuote,
    Bar,
    Caret,
    Ampersand,
    Bang,
    Hook,
    Equals,
    LessThan,
    GreaterThan,
    Plus,
    Minus,
    Tilde,
    Asterisk,
    Solidus,
    ReverseSolidus,
    Percent,
    Other,
    EoL,
    EoF
}

impl SymbolKind {
    /// Converts a character into its respective `CharKind`.
    pub fn identify(c : char) -> Self {
        match c {
            x if x.is_whitespace() => Self::Whitestuff,
            x if x.is_ascii_digit() => Self::Digit,
            x if x.is_alphanumeric() => Self::Graphic,
            '_' => Self::Underscore,
            '(' => Self::LeftParen,
            ')' => Self::RightParen,
            '{' => Self::LeftBrace,
            '}' => Self::RightBrace,
            '[' => Self::LeftBox,
            ']' => Self::RightBox,
            '.' => Self::Dot,
            ',' => Self::Comma,
            ':' => Self::Colon,
            ';' => Self::SemiColon,
            '$' => Self::Dollar,
            '`' => Self::Backtick,
            '#' => Self::Hashtag,
            '@' => Self::Address,
            '"' => Self::DoubleQuote,
            '\'' => Self::SingleQuote,
            '|' => Self::Bar,
            '^' => Self::Caret,
            '&' => Self::Ampersand,
            '!' => Self::Bang,
            '?' => Self::Hook,
            '=' => Self::Equals,
            '<' => Self::LessThan,
            '>' => Self::GreaterThan,
            '+' => Self::Plus,
            '-' => Self::Minus,
            '~' => Self::Tilde,
            '*' => Self::Asterisk,
            '/' => Self::Solidus,
            '\\' => Self::ReverseSolidus,
            '%' => Self::Percent,
            '\n' | '\r' => Self::EoL,
            _ => Self::Other
        }
    }

    /// Returns whether the symbol is a valid graphic.
    pub fn is_valid_graphic(&self) -> bool {
        matches!(self, Self::Graphic | Self::Underscore | Self::Digit)
    }

    /// Returns whether the symbol is a valid digit.
    pub fn is_valid_digit(&self) -> bool {
        matches!(self, Self::Digit)
    }

    /// Returns whether the symbol is a valid whitespace character.
    pub fn is_valid_whitespace(&self) -> bool {
        matches!(self, Self::Whitestuff | Self::EoL)
    }

    /// Returns whether the symbol is a valid terminator character.
    pub fn is_valid_terminator(&self) -> bool {
        matches!(self, Self::EoL | Self::EoF)
    }
}
