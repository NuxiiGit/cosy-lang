use std::fmt;

/// Represents a source location.
#[derive(Default, Debug, Clone)]
pub struct Span {
    pub begin : usize,
    pub end : usize,
}
impl fmt::Display for Span {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "[{}..{}]", self.begin, self.end)
    }
}
