#![allow(dead_code)]

use super::Position;

use std::fmt;
use std::error;

/// A struct which stores a compiler error with the row and column it occured on.
#[derive(Debug)]
pub struct Error {
    pub description : &'static str,
    pub position : Position
}
impl fmt::Display for Error {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        let (row, column) = self.position;
        write!(f, "(row. {}, col. {}) {}",
                row, column, self.description)
    }
}
impl error::Error for Error {}