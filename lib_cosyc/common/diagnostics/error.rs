use super::span::Span;

use std::{ fmt, error };

/// A struct which stores error information.
#[derive(Debug, Clone)]
pub struct Error {
	pub reason : &'static str,
	pub span : Span
}
impl fmt::Display for Error {
	fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
		write!(out, "{}: {}", self.span, self.reason)
    }
}
impl error::Error for Error {}