pub mod collections;
pub mod lexer;
//pub mod parser;
//pub mod interpreter;

use std::fmt;
use std::error;

/// A custom struct for defining compiler errors.
pub struct Error {
    pub title : &'static str,
    pub description : &'static str,
    pub row : usize,
    pub column : usize
}
impl fmt::Debug for Error {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({1}, {2}) - {0} ({3})",
                self.title, self.row, self.column, self.description)
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0} at (row. {1}, col. {2}): {3}",
                self.title, self.row, self.column, self.description)
    }
}
impl error::Error for Error {}