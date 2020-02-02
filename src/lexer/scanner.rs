use crate::syntax::span::Span;

use std::{
    str::CharIndices,
    iter::Peekable
};

/// A structure over a string slice which produces individual `Span`s.
pub struct Scanner<'a> {
    src : &'a str,
    chars : Peekable<CharIndices<'a>>,
    cursor_start : Cursor,
    cursor_end : Cursor
}
impl<'a> Scanner<'a> {
    /// Consume this string to create a new scanner.
    pub fn new(src : &'a str) -> Self {
        Self {
            src,
            chars : src
                    .char_indices()
                    .peekable(),
            cursor_start : Cursor::new(),
            cursor_end : Cursor::new(),
        }
    }

    /// Returns the current substring.
    pub fn substr(&self) -> &'a str {
        let start = self.cursor_start.byte;
        let end = self.cursor_end.byte;
        &self.src[start..end]
    }

    /// Clears the current substring.
    pub fn clear(&mut self) {
        self.cursor_start.row = self.cursor_end.row;
        self.cursor_start.column = self.cursor_end.column;
        self.cursor_start.byte = self.cursor_end.byte;
    }

    /// Peek at the next character. Returns `None` if the scanner is at the end of the file.
    pub fn chr(&mut self) -> Option<&char> {
        let (_, x) = self.chars.peek()?;
        Some(x)
    }

    /// Advance the cursor whilst some predicate holds.
    pub fn advance_while(&mut self, p : fn(&char) -> bool) -> &'a str {
        while let Some(x) = self.chr() {
            if !p(x) {
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
            self.cursor_end.byte = *i;
            // move cursor row/column
            if x == '\n' {
                self.cursor_end.row += 1;
                self.cursor_end.column = 1;
            } else {
                self.cursor_end.column += 1;
            }
        } else {
            // end of file
            self.cursor_end.byte = self.src.len();
        }
        Some(x)
    }

    /// Returns the span of the current substring.
    pub fn span(&self) -> Span {
        Span {
            row : self.cursor_start.row,
            column : self.cursor_start.column,
            byte_begin : self.cursor_start.byte,
            byte_end : self.cursor_end.byte
        }
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