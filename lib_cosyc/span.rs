use std::fmt;

/// A struct which pairs data with its position in the source code.
#[derive(Debug)]
pub struct Context<T> {
    value : T,
    span : Span
}

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