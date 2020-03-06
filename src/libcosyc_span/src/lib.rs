use std::fmt;

/// A struct which stores information about some substring of a source file.
#[derive(Debug)]
pub struct Span {
    pub begin : usize,
    pub end : usize,
    pub line : usize
}
impl Span {
    /// Joins two spans together to produce a new span.
    pub fn join(a : Self, b : Self) -> Self {
        Span {
            begin : a.begin,
            end : b.end,
            line : a.line
        }
    }

    /// Takes a slice out of this string which corresponds to the bytes it expects.
    pub fn render<'a>(&self, src : &'a str) -> Option<&'a str> {
        src.get(self.begin..self.end)
    }
}
impl fmt::Display for Span {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "[{}..{}] line {}",
                self.begin, self.end, self.line)
    }
}