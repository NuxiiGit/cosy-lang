#![allow(dead_code)]

use super::token::*;
use super::error::*;
use std::str::CharIndices;

/// A struct which encapsulates the state of the scanner.
pub struct Lexer<'a> {
    context : &'a str,
    scanner : CharIndices<'a>,
    errors : Vec<Error<'static>>,
    next : Option<(usize, char)>,
    row : usize,
    column : usize
}
impl<'a> Lexer<'a> {
    /// Construct a new scanner.
    pub fn lex(context : &'a str) -> Lexer<'a> {
        let mut scanner : CharIndices = context.char_indices();
        let first : Option<(usize, char)> = scanner.next();
        Lexer {
            context : context,
            scanner : scanner,
            errors : Vec::new(),
            next : first,
            row : 1,
            column : 1
        }
    }

    /// Return a slice of the current lexer errors.
    pub fn errors(&self) -> &[Error<'static>] {
        &self.errors
    }

    /// Push an error onto the error list.
    fn lexer_error(&mut self, message : &'static str) {
        self.errors.push(
                Error::new(message, self.row, self.column));
    }

    /// Create a new token with the current row and column numbers.
    fn create_token(&self, flavour : TokenType<'a>) -> Token<'a> {
        Token::new(flavour, 
                self.row, self.column)
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
                self.column = 1;
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
        loop {
            match self.peek_char() {
                // drop leading whitespace
                Some(x) if x.is_whitespace() => {
                    self.next_char();
                }
                // drop comments
                Some('\'') => {
                    self.next_char();
                    if let Some('{') = self.peek_char() {
                        // block comment
                        loop {
                            if let Some(x) = self.next_char() {
                                if x == '}' {
                                    if let Some('\'') = self.next_char() {
                                        break;
                                    }
                                }
                            } else {
                                self.lexer_error("Unclosed comment block.");
                                break;
                            }
                        }
                    } else {
                        // line comment
                        while let Some(x) = self.next_char() {
                            if x == '\n' {
                                break;
                            }
                        }
                    }
                }
                // nothing to discard
                _ => break
            }
        }
        let (c, mut start) = self.scanner.next()?;
        match c {
            // match nothing
            _ => None
        }
    }
}