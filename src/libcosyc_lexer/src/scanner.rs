use libcosyc_syntax::Context;

use std::io::{ self, BufRead, BufReader, Lines };
use std::fs::File;
use std::collections::VecDeque;

/// A structure which reads characters of a file and returns individual `Context`s.
pub struct Scanner {
    lines : Option<Lines<BufReader<File>>>,
    line : usize,
    chars : VecDeque<char>,
    word : String
}
impl Scanner {
    /// Creates a new scanner at this file path.
    pub fn open(filepath : &str) -> io::Result<Self> {
        let file = File::open(filepath)?;
        Ok(Self {
            lines : Some(BufReader::new(file).lines()),
            line : 0,
            chars : VecDeque::new(),
            word : String::new()
        })
    }

    /// Returns the kind of the next character.
    pub fn peek(&self) -> CharKind {
        if let Some(chr) = self.chr() {
            CharKind::identify(chr)
        } else {
            CharKind::EoF
        }
    }

    /// Returns the next character in the file, or `None` if you have reached the EOF.
    pub fn chr(&self) -> Option<char> {
        if self.lines.is_none() {
            None
        } else {
            if let Some(chr) = self.chars.front() {
                Some(*chr)
            } else {
                Some('\n')
            }
        }
    }

    /// Advances the scanner and adds the character to the word.
    pub fn next(&mut self) -> CharKind {
        self.advance(false)
    }

    /// Similar to `next`, except the character is ignored.
    pub fn skip(&mut self) -> CharKind {
        self.advance(true)
    }

    /// Advances the scanner.
    pub fn advance(&mut self, skip : bool) -> CharKind {
        if let Some(chr) = self.chr() {
            if let '\n' = chr {
                self.readln();
            } else {
                self.chars.pop_front();
            }
            if !skip {
                self.word.push(chr);
            }
            CharKind::identify(chr)
        } else {
            CharKind::EoF
        }
    }

    /// Returns the current substring.
    pub fn substr(&self) -> &str {
        &self.word
    }

    /// Clears the current substring.
    pub fn clear(&mut self) {
        self.word.clear();
    }

    /// Returns the current context for the current substring.
    pub fn context(&self) -> Context {
        Context {
            src : self.substr().to_string(),
            line : self.line
        }
    }

    /// Reads the next line of the file into the char queue.
    fn readln(&mut self) {
        if let Some(iter) = &mut self.lines {
            match iter.next() {
                Some(Ok(line)) => {
                    self.line += 1;
                    for x in line.chars() {
                        self.chars.push_back(x);
                    }
                },
                _ => self.lines = None
            }
        }
    }
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