use crate::syntax::token::{ Token, TokenKind};
use crate::syntax::source::Context;

use std::{ str::CharIndices, iter::Peekable };

/// A structure over a string slice which produces individual `Token`s.
pub struct Scanner<'a> {
    src : &'a str,
    chars : Peekable<CharIndices<'a>>,
    pos_start : Cursor,
    pos_end : Cursor
}
impl<'a> Scanner<'a> {
    /// Create a new scanner from this string.
    pub fn from(src : &'a str) -> Self {
        Self {
            src,
            chars : src
                    .char_indices()
                    .peekable(),
            pos_start : Cursor::new(),
            pos_end : Cursor::new()
        }
    }

    /// Returns the current substring.
    pub fn substr(&self) -> &'a str {
        let start = self.pos_start.byte;
        let end = self.pos_end.byte;
        &self.src[start..end]
    }

    /// Clears the current substring.
    pub fn clear(&mut self) {
        self.pos_start.row = self.pos_end.row;
        self.pos_start.column = self.pos_end.column;
        self.pos_start.byte = self.pos_end.byte;
    }

    /// Peek at the next character. Returns `None` if the scanner is at the end of the file.
    pub fn chr(&mut self) -> Option<&char> {
        let (_, x) = self.chars.peek()?;
        Some(x)
    }

    /// Advance the cursor whilst some predicate holds.
    pub fn advance_while(&mut self, p : fn(char) -> bool) -> &'a str {
        while let Some(x) = self.chr() {
            if !p(*x) {
                break;
            }
            self.advance();
        }
        self.substr()
    }

    /// Advance the cursor.
    pub fn advance(&mut self) -> Option<char> {
        let (_, x) = self.chars.next()?;
        if let Some((i, _)) = self.chars.peek() {
            // update span
            self.pos_end.byte = *i;
            // move cursor row/column
            if x == '\n' {
                self.pos_end.row += 1;
                self.pos_end.column = 1;
            } else {
                self.pos_end.column += 1;
            }
        } else {
            // end of file
            self.pos_end.byte = self.src.len();
        }
        Some(x)
    }

    /// Returns a token of this kind for the current substring.
    pub fn tokenise(&self, kind : TokenKind) -> Token {
        let context = Context {
            row : self.pos_start.row,
            column : self.pos_start.column,
            src : self.substr().to_string()
        };
        Token { kind, context }
    }
}

/// A container type for the current cursor position.
struct Cursor {
    pub row : usize,
    pub column : usize,
    pub byte : usize
}
impl Cursor {
    /// Creates a new default cursor.
    pub fn new() -> Self {
        Self {
            row : 0,
            column : 0,
            byte : 0
        }
    }
}