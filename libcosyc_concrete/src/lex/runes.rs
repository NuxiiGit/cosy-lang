/// Represents various kinds of character types.
#[derive(PartialEq, Eq, Debug)]
pub enum RuneKind {
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
    EoF
}

impl RuneKind {
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
            _ => Self::Other
        }
    }

    /// Returns whether the char is a valid graphic.
    pub fn is_valid_graphic(&self) -> bool {
        matches!(self, Self::Graphic)
    }

    /// Returns whether the char is a valid digit.
    pub fn is_valid_digit(&self) -> bool {
        matches!(self, Self::Digit)
    }

    /// Returns whether the char is a valid operator.
    pub fn is_valid_operator(&self) -> bool {
        matches!(self,
                Self::Dot
                | Self::Colon
                | Self::Dollar
                | Self::Hashtag
                | Self::Address
                | Self::Bar
                | Self::Ampersand
                | Self::Bang
                | Self::Hook
                | Self::Equals
                | Self::LessThan
                | Self::GreaterThan
                | Self::Plus
                | Self::Minus
                | Self::Tilde
                | Self::Asterisk
                | Self::Solidus
                | Self::ReverseSolidus
                | Self::Percent
                | Self::Other)
    }
}
