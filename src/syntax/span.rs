use std::fmt;

/// A struct which stores information about some substring of a source file.
#[derive(Debug, Clone)]
pub struct Span {
    pub byte_begin : usize,
    pub byte_end : usize,
    pub row : usize,
    pub column : usize
}
impl fmt::Display for Span {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "[{}..{}] (row. {}, col. {})",
                self.byte_begin, self.byte_end,
                self.row, self.column)
    }
}