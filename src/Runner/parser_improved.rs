#![allow(dead_code)]

use super::collections::{
    token::*,
    syntax_tree::*
};

use std::iter::Peekable;
use std::str::CharIndices;
use std::error;
use std::fmt;

/// A macro for matching a value with a pattern.
macro_rules! matches {
    ($value:expr, $($pattern:tt)*) => ({
        match $value {
            $($pattern)* => true,
            _ => false
        }
    });
}

/// A struct which encapsulates the state of the parser.
pub struct Parser<'a, I> where
        I : Iterator<Item = super::Result<Token<'a>>> {
    scanner : Peekable<I>,
    row : usize,
    column : usize
}
impl<'a, I> Parser<'a, I> where
        I : Iterator<Item = super::Result<Token<'a>>> {
    /// Constructs a new parser.
    pub fn new(scanner : I) -> Parser<'a, I> {
        Parser {
            scanner : scanner.peekable(),
            row : 0,
            column : 0
        }
    }
    
    /// Constumes the parser and produces a syntax tree of its expression.
    pub fn parse(mut self) -> Result<Expr<'a>, Vec<super::RunnerError>> {
        Err(Vec::new())
    }
}

/// An error type which represents a parser error.
#[derive(Debug)]
pub struct ParserError {
    pub description : &'static str,
    pub row : usize,
    pub column : usize
}
impl fmt::Display for ParserError {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error at (row. {}, col. {}): {}",
                self.row, self.column, self.description)
    }
}
impl error::Error for ParserError {}

/// Returns a substring of this `str`.
fn substring<'a>(s : &'a str, i : usize, n : usize) -> &'a str {
    let start : usize = i;
    let end : usize = if let Some((x, _)) = s
            .char_indices()
            .take(n + start)
            .next() {
        x
    } else {
        s.len()
    };
    &s[start..end]
}