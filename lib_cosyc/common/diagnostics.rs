use std::{ vec, slice, fmt, error };

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
	pub fn report(&mut self, location : SourcePosition, error : Error) {
		if error.kind > self.error_level {
			self.error_level = error.kind.clone();
		}
		self.errors.push(SyntaxError { location, error });
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
#[derive(Debug, Clone)]
pub struct SyntaxError {
	pub location : SourcePosition,
	pub error : Error
}

/// Represebts a source location.
pub type SourcePosition = usize;

/// Stores compile error information.
#[derive(Debug, Clone)]
pub struct Error {
	pub reason : &'static str,
	pub kind : ErrorKind
}
impl fmt::Display for Error {
	fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
		write!(out, "{:?}! {}", self.kind, self.reason)
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