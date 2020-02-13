use super::Context;

/// Stores a token and its location in the source file.
#[derive(Debug, Clone)]
pub struct Token {
    pub kind : TokenKind,
    pub context : Context
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
    Directive,
    Unknown
}
impl TokenKind {
    /// Returns `true` if the token is a keyword.
    pub fn is_keyword(&self) -> bool {
        if let TokenKind::Keyword(..) = self
                { true } else { false }
    }

    /// Returns `true` if the token is a symbol.
    pub fn is_symbol(&self) -> bool {
        if let TokenKind::Symbol(..) = self
                { true } else { false }
    }

    /// Returns `true` if the token is an identifier.
    pub fn is_identifier(&self) -> bool {
        if let
        | TokenKind::Identifier
        | TokenKind::Operator(..) = self
                { true } else { false }
    }

    /// Returns `true` if the token is an operator.
    pub fn is_operator(&self) -> bool {
        if let TokenKind::Operator(..) = self
                { false } else { true }
    }

    /// Returns `true` if the token is a literal.
    pub fn is_literal(&self) -> bool {
        if let TokenKind::Literal(..) = self
                { true } else { false }
    }

    /// Returns `true` if the token is the end of the file.
    pub fn is_eof(&self) -> bool {
        if let TokenKind::EoF = self
                { true } else { false }
    }

    /// Returns `true` if the token is a compiler directive.
    pub fn is_directive(&self) -> bool {
        if let TokenKind::Directive = self
                { true } else { false }
    }

    /// Returns `true` if the token is unknown.
    pub fn is_unknown(&self) -> bool {
        if let TokenKind::Unknown = self
                { true } else { false }
    }

    /// Returns `true` if the token is a valid non-terminal.
    pub fn is_nonterminal(&self) -> bool {
        self.is_identifier() ||
        self.is_literal() ||
        self.is_directive()
    }
}

/// An enum which describes available keyword types.
#[derive(PartialEq, Debug, Clone)]
pub enum KeywordKind {
    Var,
    If,
    Else
}

/// An enum which describes available symbol types.
#[derive(PartialEq, Debug, Clone)]
pub enum SymbolKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    SemiColon,
    Dollar,
    Backtick,
    Address
}

/// An enum which describes available identifier types.
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
    Percent,
    Other
}

/// An enum which describes available literal types.
#[derive(PartialEq, Debug, Clone)]
pub enum LiteralKind {
    Character,
    Integer,
    Real
}

/// An enum which stores character kinds.
#[derive(PartialEq, Debug, Clone)]
pub enum CharKind {
    NewLine,
    Whitespace,
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
    ForwardSlash,
    BackSlash,
    Percent,
    EoF,
    Other
}
impl CharKind {
    /// Converts a character into its respective `CharKind`.
    pub fn identify(c : char) -> CharKind {
        match c {
            '\n' => CharKind::NewLine,
            x if x.is_whitespace() => CharKind::Whitespace,
            x if x.is_ascii_digit() => CharKind::Digit,
            x if x.is_alphanumeric() => CharKind::Graphic,
            '_' => CharKind::Underscore,
            '(' => CharKind::LeftParen,
            ')' => CharKind::RightParen,
            '{' => CharKind::LeftBrace,
            '}' => CharKind::RightBrace,
            '[' => CharKind::LeftBox,
            ']' => CharKind::RightBox,
            '.' => CharKind::Dot,
            ',' => CharKind::Comma,
            ':' => CharKind::Colon,
            ';' => CharKind::SemiColon,
            '$' => CharKind::Dollar,
            '`' => CharKind::Backtick,
            '#' => CharKind::Hashtag,
            '@' => CharKind::Address,
            '"' => CharKind::DoubleQuote,
            '\'' => CharKind::SingleQuote,
            | '|'
            | '¦' => CharKind::Bar,
            '^' => CharKind::Caret,
            '&' => CharKind::Ampersand,
            '!' => CharKind::Bang,
            '?' => CharKind::Hook,
            '=' => CharKind::Equals,
            '<' => CharKind::LessThan,
            '>' => CharKind::GreaterThan,
            '+' => CharKind::Plus,
            '-' => CharKind::Minus,
            '~' => CharKind::Tilde,
            '*' => CharKind::Asterisk,
            '/' => CharKind::ForwardSlash,
            '\\' => CharKind::BackSlash,
            '%' => CharKind::Percent,
            _ => CharKind::Other
        }
    }

    /// Returns whether the char is valid whitespace.
    pub fn is_valid_whitespace(&self) -> bool {
        if let
        | CharKind::NewLine
        | CharKind::Whitespace = self {
            true
        } else {
            false
        }
    }

    /// Returns whether the char is a valid digit.
    pub fn is_valid_digit(&self) -> bool {
        if let
        | CharKind::Digit = self {
            true
        } else {
            false
        }
    }

    /// Returns whether the char is a valid graphic.
    pub fn is_valid_graphic(&self) -> bool {
        if let
        | CharKind::Graphic
        | CharKind::Underscore
        | CharKind::SingleQuote = self {
            true
        } else {
            false
        }
    }

    /// Returns whether the char is a valid operator.
    pub fn is_valid_operator(&self) -> bool {
        if let
        | CharKind::Bar
        | CharKind::Caret
        | CharKind::Ampersand
        | CharKind::Bang
        | CharKind::Hook
        | CharKind::Equals
        | CharKind::LessThan
        | CharKind::GreaterThan
        | CharKind::Plus
        | CharKind::Minus
        | CharKind::Tilde
        | CharKind::Asterisk
        | CharKind::ForwardSlash
        | CharKind::BackSlash
        | CharKind::Percent
        | CharKind::Other = self {
            true
        } else {
            false
        }
    }

    /// Returns whether the char is a valid line ending.
    pub fn is_valid_ending(&self) -> bool {
        if let
        | CharKind::NewLine
        | CharKind::EoF = self {
            true
        } else {
            false
        }
    }
}