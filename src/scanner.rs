use super::error::Error;
use super::token::{
    Token,
    TokenKind,
    IdentifierKind,
    LiteralKind
};

use std::iter::Peekable;
use std::str::CharIndices;

/// An iterator over a string slice, which produces `Token`s.
pub struct Lexer<'a> {
    scanner : StrScanner<'a>
}
impl<'a> Lexer<'a> {
    /// Create a new lexer.
    pub fn new(scanner : StrScanner<'a>) -> Lexer<'a> {
        Lexer { scanner }
    }
}
impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, Error<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

/// A structure over a string slice which produces individual `Span`s of tokens.
pub struct StrScanner<'a> {
    context : &'a str,
    chars : Peekable<CharIndices<'a>>,
    row : usize,
    column : usize
}
impl<'a> StrScanner<'a> {
    /// Create a new scanner from this string slice.
    pub fn from(context : &'a str) -> StrScanner<'a> {
        StrScanner {
            context,
            chars : context
                    .char_indices()
                    .peekable(),
            row : 1,
            column : 0
        }
    }

    /// Returns the current column of the scanner.
    fn column(&self) -> usize {
        self.column
    }

    /// Returns the current row of the scanner.
    fn row(&self) -> usize {
        self.row
    }

    /// Peek at the next character.
    fn chr(&mut self) -> Option<char> {
        let (_, x) = self.chars.peek()?;
        Some(*x)
    }

    /// Peek at the next index.
    /// Returns `str.len()` if the end is reached.
    fn pos(&mut self) -> usize {
        if let Some((i, _)) = self.chars.peek() {
            *i
        } else {
            self.context.len()
        }
    }

    /// Move to the next character.
    fn advance(&mut self) -> Option<char> {
        let (_, x) = self.chars.next()?;
        if x == '\n' {
            // move to new line
            self.row += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(x)
    }
}