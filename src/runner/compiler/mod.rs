pub mod parser;
pub mod scanner;

use std::fmt;
use std::error;
use std::result;

/// A struct which describes compiler errors.
#[derive(Debug)]
pub struct Error {
    pub description : &'static str,
    pub row : usize,
    pub column : usize
}
impl fmt::Display for Error {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Compile error at (row. {}, col. {}): {}",
                self.row, self.column, self.description)
    }
}
impl error::Error for Error {}

/// A custom result for compile errors.
pub type Result<T> = result::Result<T, Vec<Error>>;