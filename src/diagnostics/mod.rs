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