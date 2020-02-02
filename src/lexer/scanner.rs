use crate::syntax::span::Span;

use std::{
    str::CharIndices,
    iter::Peekable
};

/// A structure over a string slice which produces individual `Span`s.
pub struct Cursor<'a> {
    src : &'a str,
    chars : Peekable<CharIndices<'a>>,
    pos_start : SlicePosition,
    pos_end : SlicePosition
}
impl<'a> Cursor<'a> {
    /// Consume this string to create a new scanner.
    pub fn new(src : &'a str) -> Self {
        Self {
            src,
            chars : src
                    .char_indices()
                    .peekable(),
            pos_start : SlicePosition::new(),
            pos_end : SlicePosition::new(),
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

    /// Returns the span of the current substring.
    pub fn span(&self) -> Span {
        Span {
            row : self.pos_start.row,
            column : self.pos_start.column,
            byte_begin : self.pos_start.byte,
            byte_end : self.pos_end.byte
        }
    }
}

/// A container type for the current cursor position.
struct SlicePosition {
    pub row : usize,
    pub column : usize,
    pub byte : usize
}
impl SlicePosition {
    /// Creates a new default cursor.
    pub fn new() -> Self {
        Self {
            row : 0,
            column : 0,
            byte : 0
        }
    }
}