use std::fmt;

/// A struct which stores information about some substring of a source file.
#[derive(Debug, Clone)]
pub struct Context {
    pub src : String,
    pub row : usize,
    pub column : usize
}
impl fmt::Display for Context {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "(row. {}, col. {})", self.row, self.column)
    }
}