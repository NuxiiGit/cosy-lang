#![allow(dead_code)]

use super::token::*;
use super::error::*;
use std::str::CharIndices;

/// A struct which encapsulates the state of the scanner.
pub struct Lexer<'a> {
    context : &'a str,
    scanner : CharIndices<'a>,
    errors : &'a mut Vec<Error<'a>>,
    next : Option<(usize, char)>,
    row : usize,
    column : usize
}
impl<'a> Lexer<'a> {
    /// Construct a new scanner.
    pub fn new(context : &'a str, errors : &'a mut Vec<Error<'a>>) -> Lexer<'a> {
        let mut scanner : CharIndices = context.char_indices();
        let first : Option<(usize, char)> = scanner.next();
        Lexer {
            context : context,
            scanner : scanner,
            errors : errors,
            next : first,
            row : 0,
            column : 0
        }
    }

    /// Push an error onto the error list.
    fn lexer_error(&mut self, message : &'static str) {
        self.errors.push(Error::new(message,
                self.row, self.column));
    }

    /// Create a new token with the current row and column numbers.
    fn create_token(&self, flavour : TokenType<'a>) -> Token<'a> {
        Token::new(flavour, 
                self.row, self.column)
    }

    /// Drop the whitespace.
    fn drop_whitespace(&mut self) {
        while let Some(x) = self.peek_char() {
            if x.is_whitespace() {
                self.next_char();
            } else {
                break;
            }
        }
    }

    /// Move to the next character.
    fn next_char(&mut self) -> Option<char> {
        let (_, x) = self.next_charindex()?;
        Some(x)
    }

    /// Move to the next character-index pair.
    fn next_charindex(&mut self) -> Option<(usize, char)> {
        let next : Option<(usize, char)> = self.next;
        if let Some((_, x)) = next {
            if let '\n' = x {
                self.row += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }
            self.next = self.scanner.next();
        }
        next
    }

    /// Peek at the next character.
    fn peek_char(&self) -> Option<char> {
        let (_, x) = self.peek_charindex()?;
        Some(x)
    }

    /// Peek at the next index. Returns `context.len()` if the end is reached.
    fn peek_index(&self) -> usize {
        if let Some((i, _)) = self.peek_charindex() {
            i
        } else {
            self.context.len()
        }
    }

    /// Peek at the next character-index pair.
    fn peek_charindex(&self) -> Option<(usize, char)> {
        self.next
    }
}
impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.drop_whitespace();
        let (c, mut start) = self.scanner.next()?;
        match c {
            // match nothing
            _ => None
        }
    }
}