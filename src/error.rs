use super::source_pos::Span;

use std::fmt;
use std::error;

/// A struct which stores error information.
#[derive(Debug)]
pub struct Error<'a> {
    pub reason : &'static str,
    pub span : Span<'a>
}
impl fmt::Display for Error<'_> {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "Error! {}: {}",
                self.span, self.reason)
    }
}
impl error::Error for Error<'_> {}