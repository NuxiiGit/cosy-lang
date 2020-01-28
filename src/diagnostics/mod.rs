use crate::syntax::span::Span;

use std::fmt;
use std::error;

/// A struct which stores error information.
#[derive(Debug)]
pub struct Error {
    pub reason : &'static str,
    pub span : Span
}
impl fmt::Display for Error {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "error {}: {}", self.span, self.reason)
    }
}
impl error::Error for Error {}