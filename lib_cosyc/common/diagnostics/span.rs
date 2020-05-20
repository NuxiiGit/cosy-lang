use std::fmt;

/// Represents information about some substring of a source file.
#[derive(Debug, Clone, Default)]
pub struct Span {
	pub begin : Cursor,
	pub end : Cursor
}
impl Span {
	/// Creates a default span
	pub fn new() -> Self {
		Self::default()
	}

	/// Joins two spans together to produce a new span.
	pub fn join(&self, other: &Self) -> Self {
		Span {
			begin : self.begin.clone(),
			end : other.end.clone()
		}
	}
}
impl fmt::Display for Span {
	fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
		write!(out, "[{}..{}] (row. {}, col. {})", 
				self.begin.byte, self.end.byte,
				self.begin.line + 1, self.begin.column + 1)
	}
}

/// Represents a position in a file.
#[derive(Debug, Clone, Default)]
pub struct Cursor {
	pub byte : usize,
	pub line : usize,
	/// The number of UTF-8 codepoints since the last new line.
	pub column : usize
}