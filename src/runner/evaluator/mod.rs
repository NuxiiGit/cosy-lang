pub mod interpreter;

use std::fmt;
use std::error;
use std::result;

/// An enum which describes the different types of value.
#[derive(Debug)]
pub enum Value {
    None,
    Bool(bool),
    Char(char),
    Integer(i64),
    Float(f64)
}

/// A struct which describes runtime errors.
#[derive(Debug)]
pub struct Error {
    pub description : &'static str,
    pub row : usize,
    pub column : usize
}
impl fmt::Display for Error {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Runtime error at (row. {}, col. {}): {}",
                self.row, self.column, self.description)?;
        Ok(())
    }
}
impl error::Error for Error {}