use crate::syntax::token::Token;

use std::fmt;
use std::error;

/// A struct which stores error information.
#[derive(Debug)]
pub struct Error {
    pub reason : &'static str,
    pub token : Token
}
impl fmt::Display for Error {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "error {}: {}. got {:?}",
                self.token.span, self.reason, self.token.kind)
    }
}
impl error::Error for Error {}