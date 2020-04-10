use std::fmt;

/// A struct which stores information about some substring of a source file.
#[derive(Debug, Clone)]
pub struct Span {
	pub begin : usize,
	pub end : usize,
	pub line : usize
}
impl Span {
	/// Creates a default span
	pub fn new() -> Self {
		Span {
			begin : 0,
			end : 0,
			line : 1
		}
	}

	/// Joins two spans together to produce a new span.
	pub fn join(&self, other: &Self) -> Self {
		Span {
			begin : self.begin,
			end : other.end,
			line : self.line
		}
	}

	/// Copies the values of `other` into `self`
	pub fn replicate(&mut self, other: &Self) {
		self.begin = other.begin;
		self.end = other.end;
		self.line = other.line;
	}

	/// Takes a slice out of this string which corresponds to the bytes it expects.
	pub fn render<'a>(&self, src : &'a str) -> Option<&'a str> {
		src.get(self.begin..self.end)
	}
}
impl fmt::Display for Span {
	fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
		write!(out, "[{}..{}] line {}", self.begin, self.end, self.line)
	}
}