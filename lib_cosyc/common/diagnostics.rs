pub mod error;

use error::{ Error, ErrorKind };

use std::{ vec, slice };

/// A struct which keeps track of errors.
#[derive(Default)]
pub struct IssueTracker {
	errors : Vec<SyntaxError>,
	pub error_level : ErrorKind
}
impl IssueTracker {
	/// Creates a new empty session.
	pub fn new() -> Self {
		Self::default()
	}

	/// Returns whether errors occurred.
	pub fn contains_errors(&self) -> bool {
		!self.errors.is_empty()
	}

	/// Adds a new error to the session.
	pub fn report(&mut self, location : usize, error : Error) {
		if error.kind > self.error_level {
			self.error_level = error.kind.clone();
		}
		self.errors.push((location, error));
	}
}
impl IntoIterator for IssueTracker {
	type Item = SyntaxError;
	type IntoIter = vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.errors.into_iter()
	}
}
impl<'a> IntoIterator for &'a IssueTracker {
	type Item = &'a SyntaxError;
	type IntoIter = slice::Iter<'a, SyntaxError>;

	fn into_iter(self) -> Self::IntoIter {
		(&self.errors).into_iter()
	}
}

/// Represents an error and the location it occurred in the source file.
pub type SyntaxError = (usize, Error);