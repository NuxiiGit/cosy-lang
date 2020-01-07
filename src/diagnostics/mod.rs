use crate::syntax::token::Token;

use std::fmt;
use std::error;

/// A struct which stores error information.
#[derive(Debug)]
pub struct Error<'a> {
    pub reason : &'static str,
    pub token : Token<'a>
}
impl fmt::Display for Error<'_> {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "{}: {}",
                self.token.span, self.reason)
    }
}
impl error::Error for Error<'_> {}

/// A struct which stores information about some substring of a source file.
#[derive(Debug, Clone)]
pub struct Span<'a> {
    pub content : &'a str,
    pub row : usize,
    pub column : usize
}
impl fmt::Display for Span<'_> {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "got '{}' at (row: {}, col: {})",
                self.content, self.row, self.column)
    }
}