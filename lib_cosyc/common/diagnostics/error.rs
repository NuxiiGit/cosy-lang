use super::span::Span;

use std::{ fmt, error };

/// Stores compile error information.
#[derive(Debug, Clone)]
pub struct Error {
	pub reason : &'static str,
	pub span : Span,
	pub kind : ErrorKind
}
impl fmt::Display for Error {
	fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
		write!(out, "{:?}! {}: {}", self.kind, self.span, self.reason)
    }
}
impl error::Error for Error {}

/// Represents the differnt kinds of error.
#[derive(PartialOrd, PartialEq, Debug, Clone)]
pub enum ErrorKind {
    Warning,
    Fatal
}
impl Default for ErrorKind {
	fn default() -> Self {
		ErrorKind::Warning
	}
}