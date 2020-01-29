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
    cursor_end : Cursor,
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
            cursor_end : Cursor::new()
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
        self.cursor_end.copy_into(&mut self.cursor_start);
    }

    /// Peek at the next character.
    pub fn chr(&mut self) -> Option<char> {
        let (_, x) = self.chars.peek()?;
        Some(*x)
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

    /// Consumes the current substring and returns its span.
    pub fn consume(&mut self) -> Span {
        let span = Span {
            row : self.cursor_start.row,
            column : self.cursor_start.column,
            byte_begin : self.cursor_start.byte,
            byte_end : self.cursor_end.byte
        };
        self.clear();
        span
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

    /// Copies the state of this cursor into another.
    pub fn copy_into(&self, cursor : &mut Cursor) {
        cursor.row = self.row;
        cursor.column = self.column;
        cursor.byte = self.byte;
    }
}