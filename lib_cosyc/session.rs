use crate::diagnostics::IssueTracker;
use crate::span::Span;

use std::str::CharIndices;
use std::mem;

/// A struct which stores session information, such as:
/// - Source code
/// - Character stream
/// - Errors
pub struct Session<'a> {
    src : &'a str,
    chars : CharIndices<'a>,
    peek : CharKind,
    span : Span,
    /// Used to log any errors encountered during the session.
    pub issues : IssueTracker
}
impl<'a> Session<'a> {
    /// Creates a new parser session from this source code.
    pub fn from(src : &'a str) -> Self {
        let mut chars = src.char_indices();
        let first = chars
                .next()
                .map(|(_, c)| CharKind::identify(c))
                .unwrap_or(CharKind::EoF);
        Self {
            src,
            chars,
            peek : first,
            span : Span {
                line : 1,
                begin : 0,
                end : 0
            },
            issues : IssueTracker::new()
        }
    }

    /// Returns the current peeked character.
    pub fn peek(&self) -> &CharKind {
        &self.peek
    }

    /// Advances the session scanner.
    pub fn next(&mut self) -> CharKind {
        if self.peek.is_valid_newline() {
            self.span.line += 1;
        }
        let next = if let Some((i, c)) = self.chars.next() {
            self.span.end = i;
            CharKind::identify(c)
        } else {
            self.span.end = self.src.len();
            CharKind::EoF
        };
        mem::replace(&mut self.peek, next)
    }

    /// Returns the current substring.
    pub fn substr(&self) -> &'a str {
        &self.src[self.span.begin..self.span.end]
    }

    /// Clears the current substring.
    pub fn clear_substr(&mut self) {
        self.span.begin = self.span.end;
    }

    /// Returns the source code as a slice.
    pub fn src(&self) -> &'a str {
        &self.src
    }
    
    /// Returns the current span.
    pub fn span(&self) -> Span {
        self.span.clone()
    }
}

/// An enum which stores character kinds.
#[derive(PartialEq, Debug, Clone)]
pub enum CharKind {
    Lf,
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
            '\n' => CharKind::Lf,
            x if x.is_whitespace() => CharKind::Space,
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
        | CharKind::Space = self {
            true
        } else {
            self.is_valid_newline()
        }
    }

    /// Returns whether the char is a valid line ending.
    pub fn is_valid_ending(&self) -> bool {
        if let
        | CharKind::EoF = self {
            true
        } else {
            self.is_valid_newline()
        }
    }

    /// Returns whether the char is valid new line character.
    pub fn is_valid_newline(&self) -> bool {
        if let
        | CharKind::Lf = self {
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
}