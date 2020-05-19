pub mod error;
pub mod span;

use error::Error;
use span::Span;

use std::{ fmt, vec, slice };

/// A struct which keeps track of errors.
#[derive(Default)]
pub struct IssueTracker {
	errors : Vec<Node<Error>>
}
impl IssueTracker {
	/// Creates a new empty session.
	pub fn new() -> Self {
		Self::default()
	}

	/// Adds a new error to the session.
	pub fn report(&mut self, error : Node<Error>) {
		self.errors.push(error);
	}
}
impl IntoIterator for IssueTracker {
	type Item = Node<Error>;
	type IntoIter = vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.errors.into_iter()
	}
}
impl<'a> IntoIterator for &'a IssueTracker {
	type Item = &'a Node<Error>;
	type IntoIter = slice::Iter<'a, Node<Error>>;

	fn into_iter(self) -> Self::IntoIter {
		(&self.errors).into_iter()
	}
}

/// Represents a piece of data paired with a source position.
#[derive(Clone, Debug)]
pub struct Node<T : fmt::Debug + Clone> {
	pub content : T,
	pub span : Span
}
impl<T : fmt::Debug + Clone> fmt::Display for Node<T> {
	fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
		write!(out, "{}: {:?}", self.span, self.content)
	}
}
impl<T : fmt::Debug + Clone> From<T> for Node<T> {
	fn from(content : T) -> Self {
		Self { content, span : Span::default() }
	}
}