use std::fmt;

/// A struct which stores information about some substring of a source file.
#[derive(Debug, Clone)]
pub struct Span<'a> {
    pub content : &'a str,
    pub row : usize,
    pub column : usize
}
impl fmt::Display for Span<'_> {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "got '{}' at (row: {}, col: {})",
                self.content, self.row, self.column)
    }
}