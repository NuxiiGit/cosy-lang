use crate::span::Span;

use std::{ fmt, error };
use std::vec;

/// A struct which keeps track of errors.
pub struct IssueTracker {
	errors : Vec<Error>,
	level : Option<ErrorKind>
}
impl IssueTracker {
	/// Creates a new empty session.
	pub fn new() -> Self {
		Self {
			errors : Vec::new(),
			level : None
		}
	}

	/// Returns the error level, if one exists.
	pub fn level(&self) -> Option<&ErrorKind> {
		if let Some(error) = &self.level
				{ Some(error) } else { None }
	}

	/// Adds a new error to the session.
	pub fn report(&mut self, error : Error) {
		let increase_level = if let Some(kind) = &self.level
				{ error.kind > *kind } else { true };
		if increase_level {
			self.level = Some(error.kind.clone());
		}
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

/// Additional implementations of the `Span` struct to support easier conversion to errors.
impl Span {
	/// Creates an error of this kind and reaosn.
	pub fn make_error(&self, kind : ErrorKind, reason : &'static str) -> Error {
		let span = self.clone();
		Error { reason, kind, span }
	}
}

/// A struct which stores error information.
#[derive(Debug)]
pub struct Error {
	pub reason : &'static str,
	pub kind : ErrorKind,
	pub span : Span
}
impl fmt::Display for Error {
	fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
		write!(out, "{:?}! {}: {}", self.kind, self.span, self.reason)
    }
}
impl error::Error for Error {}

/// An enum which describes available error types.
#[derive(PartialOrd, PartialEq, Debug, Clone)]
pub enum ErrorKind {
	Warning,
	Fatal
}