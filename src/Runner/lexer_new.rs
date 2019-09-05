#![allow(dead_code)]

use super::token::*;
use super::error::*;
use std::str::CharIndices;
use std::iter::Peekable;

/// A struct which encapsulates the state of the scanner.
pub struct Lexer<'a> {
    scanner : Peekable<Scanner<'a>>,
    row : usize,
    column : usize,
    errors : &'a mut Vec<Error<'a>>
}
impl<'a> Lexer<'a> {
    /// Construct a new scanner.
    pub fn new(context : &'a str, errors : &'a mut Vec<Error<'a>>) -> Lexer<'a> {
        Lexer {
            scanner : Scanner::new(context)
                    .peekable(),
            row : 0,
            column : 0,
            errors : errors
        }
    }

    /// Push an error onto the error list.
    fn lexer_error(&mut self, message : &'static str) {
        self.errors.push(Error::new(message, self.row, self.column));
    }

    /// Create a new token with the current row and column numbers.
    fn create_token(&self, flavour : TokenType<'a>) -> Token<'a> {
        Token::new(flavour, self.row, self.column)
    }

    /// Drop the whitespace.
    fn drop_whitespace(&mut self) {
        while let Some((_, c)) = self.scanner.peek() {
            if c.is_whitespace() {
                self.scanner.next();
            } else {
                break;
            }
        }
    }
}
impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.drop_whitespace();
        let (start, c) : (usize, char) = self.scanner.next()?;
        match c {
            // match nothing
            _ => None
        }
    }
}

/// A struct which allows you to iterate and slice `&str` types.
pub struct Scanner<'a> {
    context : &'a str,
    chars : CharIndices<'a>,
    row : usize,
    column : usize
}
impl<'a> Scanner<'a> {
    /// Construct a new scanner.
    pub fn new(context : &str) -> Scanner {
        Scanner {
            context : context,
            chars : context.char_indices(),
            row : 0,
            column : 0,
        }
    }

    /// Returns a slice of the scanner context.
    pub fn substring(&self, start : usize, end : usize) -> &'a str {
        &self.context[start..end]
    }

    /// Returns the current row of the scanner.
    pub fn row(&mut self) -> usize {
        self.row
    }

    /// Returns the current column of the scanner.
    pub fn column(&mut self) -> usize {
        self.column
    }
}
impl<'a> Iterator for Scanner<'a> {
    type Item = (usize, char);

    fn next(&mut self) -> Option<Self::Item> {
        let next : (usize, char) = self.chars.next()?;
        if let (_, '\n') = next {
            self.row += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
        Some(next)
    }
}