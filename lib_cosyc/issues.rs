use crate::span::Span;

use std::{ fmt, error };
use std::vec;

/// A struct which keeps track of errors.
pub struct IssueTracker {
	errors : Vec<Error>
}
impl IssueTracker {
	/// Creates a new empty session.
	pub fn new() -> Self {
		Self {
			errors : Vec::new(),
		}
	}

	/// Adds a new error to the session.
	pub fn report(&mut self, error : Error) {
		self.errors.push(error);
	}
}
impl IntoIterator for IssueTracker {
	type Item = Error;
	type IntoIter = vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.errors.into_iter()
	}
}

/// A struct which stores error information.
#[derive(Debug)]
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