use crate::syntax::token::Token;

use std::fmt;
use std::error;

/// A struct which stores error information.
#[derive(Debug)]
pub struct Error<'a> {
    pub kind : ErrorKind,
    pub reason : &'static str,
    pub token : Token<'a>
}
impl Error<'_> {
    /// Returns whether this error is fatal.
    pub fn is_fatal(&self) -> bool {
        self.kind == ErrorKind::Fatal
    }
}
impl fmt::Display for Error<'_> {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "{:?}! {}: {}",
                self.kind, self.token.span, self.reason)
    }
}
impl error::Error for Error<'_> {}

/// An enum for error types.
#[derive(PartialEq, Debug)]
pub enum ErrorKind {
    Warning,
    Fatal
}

/// A struct which stores information about some substring of a source file.
#[derive(Debug)]
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