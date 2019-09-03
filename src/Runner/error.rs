#![allow(dead_code)]

/// A struct which stores parser error information, such as the line and column numbers.
pub struct CompileError<'a> {
    msg : &'a str,
    row : usize,
    col : usize
}
impl<'a> CompileError<'a> {
    /// Constructs a new compiler error with these line and column numbers.
    pub fn new(msg : &str, row : usize, col : usize) -> CompileError {
        CompileError {
            msg : msg,
            row : row,
            col : col
        }
    }

    /// Returns the error message.
    pub fn message(&self) -> &'a str {
        self.msg
    }

    /// Returns the line number.
    pub fn row(&self) -> usize {
        self.row
    }

    /// Returns the column number.
    pub fn column(&self) -> usize {
        self.row
    }
}