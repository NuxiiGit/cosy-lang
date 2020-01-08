use crate::syntax::token::Token;

use std::fmt;
use std::error;

/// A struct which handles the compilation of errors.
#[derive(Debug)]
pub struct Handler<'a> {
    errors : Vec<Error<'a>>
}
impl Handler<'_> {
    /// Creates an empty error handler.
    pub fn new() -> Self {
        Handler {
            errors : Vec::new()
        }
    }
}
impl fmt::Display for Handler<'_> {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        if self.errors.len() == 0 {
            write!(out, "No errors!")
        } else {
            write!(out, "Errors:{}", self.errors.iter().fold(String::new(), |mut acc, err| {
                acc.push('\n');
                acc.push_str(&err.to_string());
                acc
            }))
        }
    }
}
impl error::Error for Handler<'_> {}

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