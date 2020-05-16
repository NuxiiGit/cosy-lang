use std::fmt;

/// Represents information about some substring of a source file.
#[derive(Debug, Clone, Default)]
pub struct Span {
	pub begin : usize,
	pub end : usize,
	pub line : usize
}
impl Span {
	/// Creates a default span
	pub fn new() -> Self {
		Self::default()
	}

	/// Joins two spans together to produce a new span.
	pub fn join(&self, other: &Self) -> Self {
		Span {
			begin : self.begin,
			end : other.end,
			line : self.line
		}
	}
}
impl fmt::Display for Span {
	fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
		write!(out, "[{}..{}] line {}", self.begin, self.end, self.line + 1)
	}
}