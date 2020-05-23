use std::{ fmt, cmp };

/// Represents information about some substring of a source file.
#[derive(Debug, Clone, Default)]
pub struct Span {
	pub start : usize,
	pub end : usize
}
impl Span {
	/// Creates a default span
	pub fn new() -> Self {
		Self::default()
	}

	/// Joins two spans together to produce a new span.
	pub fn join(&self, other: &Self) -> Self {
		let start = cmp::min(self.start, other.start);
		let end = cmp::min(self.end, other.end);
		Span { start, end }
	}

	/// Calculates the length of this span.
	/// A negative number is returned in the case that `start > end`.
	pub fn length(&self) -> usize {
		self.end - self.start
	}
}
impl fmt::Display for Span {
	fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
		write!(out, "[{}..{}]", self.start, self.end)
	}
}