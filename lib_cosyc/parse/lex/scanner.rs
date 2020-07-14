use crate::common::diagnostics::SourcePosition;

use std::{ str::CharIndices, mem };

/// Represents various kinds of character types.
#[derive(PartialEq, Debug, Clone)]
pub enum CharKind {
    EoL,
    Tab,
    Space,
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
    /// Any other unicode character or symbol.
    Other,
    EoF
}
impl CharKind {
    /// Converts a character into its respective `CharKind`.
    pub fn identify(c : char) -> CharKind {
        use CharKind::*;
        match c {
            '\n' => EoL,
            '\t' => Tab,
            x if x.is_whitespace() => Space,
            x if x.is_ascii_digit() => Digit,
            x if x.is_alphanumeric() => Graphic,
            '_' => Underscore,
            '(' => LeftParen,
            ')' => RightParen,
            '{' => LeftBrace,
            '}' => RightBrace,
            '[' => LeftBox,
            ']' => RightBox,
            '.' => Dot,
            ',' => Comma,
            ':' => Colon,
            ';' => SemiColon,
            '$' => Dollar,
            '`' => Backtick,
            '#' => Hashtag,
            '@' => Address,
            '"' => DoubleQuote,
            '\'' => SingleQuote,
            '|'
            | '¦' => Bar,
            '^' => Caret,
            '&' => Ampersand,
            '!' => Bang,
            '?' => Hook,
            '=' => Equals,
            '<' => LessThan,
            '>' => GreaterThan,
            '+' => Plus,
            '-' => Minus,
            '~' => Tilde,
            '*' => Asterisk,
            '/' => Solidus,
            '\\' => ReverseSolidus,
            '%' => Percent,
            _ => Other
        }
    }

    /// Returns whether the char is valid whitespace.
    pub fn is_valid_whitespace(&self) -> bool {
        use CharKind::*;
        matches!(self, Tab | Space) || self.is_valid_newline()
    }

    /// Returns whether the char is valid new line character.
    pub fn is_valid_newline(&self) -> bool {
        matches!(self, CharKind::EoL)
    }

    /// Returns whether the char is a valid graphic.
    pub fn is_valid_graphic(&self) -> bool {
        matches!(self, CharKind::Graphic) || self.is_valid_digit()
    }

    /// Returns whether the char is a valid digit.
    pub fn is_valid_digit(&self) -> bool {
        matches!(self, CharKind::Digit)
    }

    /// Returns whether the char is a valid operator.
    pub fn is_valid_operator(&self) -> bool {
        use CharKind::*;
        matches!(self,
                Dot
                | Colon
                | Dollar
                | Hashtag
                | Address
                | Bar
                | Ampersand
                | Bang
                | Hook
                | Equals
                | LessThan
                | GreaterThan
                | Plus
                | Minus
                | Tilde
                | Asterisk
                | Solidus
                | ReverseSolidus
                | Percent
                | Other)
    }
}

/// Iterates over characters of a string, producing useful substrings and tagged data.
pub struct CharReader<'a> {
    src : &'a str,
    chars : CharIndices<'a>,
    current : CharKind,
    span_begin : SourcePosition,
    span_end : SourcePosition,
    /// Used to detect comment lexemes.
    only_dashes : bool
}
impl<'a> CharReader<'a> {
    /// Advances the reader whilst some predicate holds.
    /// Always halts if the `EoF` character is reached.
    pub fn advance_while(&mut self, p : fn(&CharKind) -> bool) {
        loop {
            match self.current() {
                CharKind::EoF => break,
                x if p(x) => { self.advance(); },
                _ => break
            }
        }
    }

    /// Peeks at the next `CharKind` in the string.
    pub fn current(&self) -> &CharKind {
        &self.current
    }

    /// Advances the reader and returns the next `CharKind`.
    pub fn advance(&mut self) -> CharKind {
        let future = if let Some((i, c)) = self.chars.next() {
            self.span_end = i;
            CharKind::identify(c)
        } else {
            self.span_end = self.src.len();
            CharKind::EoF
        };
        if self.only_dashes && !matches!(self.current, CharKind::Minus) {
            self.only_dashes = false;
        }
        mem::replace(&mut self.current, future)
    }

    /// Returns the current substring.
    pub fn slice(&self) -> &'a str {
        &self.src[self.span_begin..self.span_end]
    }

    /// Clears the current span.
    pub fn reset_span(&mut self) {
        self.span_begin = self.span_end;
        self.only_dashes = true;
    }

    /// Returns the current cursor position.
    pub fn cursor(&self) -> SourcePosition {
        self.span_begin
    }

    /// Returns whether the current stream of characters is a comment lexeme.
    pub fn holds_comment_lexeme(&self) -> bool {
        self.only_dashes && self.span_end - self.span_begin > 1
    }
}
impl<'a> From<&'a str> for CharReader<'a> {
    fn from(src : &'a str) -> Self {
        let mut chars = src.char_indices();
        let current = chars
                .next()
                .map(|(_, snd)| CharKind::identify(snd))
                .unwrap_or(CharKind::EoF);
        Self {
            src,
            chars,
            current,
            span_begin : 0,
            span_end : 0,
            only_dashes : true
        }
    }
}
