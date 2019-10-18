pub mod parser;
pub mod scanner;

use std::fmt;
use std::error;

/// An enum which describes compiler errors.
#[derive(Debug)]
pub struct Error {
    description : &'static str,
    row : usize,
    column : usize
}
impl fmt::Display for Error {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Compile error at (row. {}, col. {}): {}",
                self.row, self.column, self.description)
    }
}
impl error::Error for Error {}