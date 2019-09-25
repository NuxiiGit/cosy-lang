#![allow(dead_code)]

use std::fmt;

static mut ERRORS : Option<Vec<Error>> = None;

/// A struct which holds error information.
pub struct Error {
    pub message : &'static str,
    pub row : usize,
    pub column : usize
}
impl Error {
    /// Returns the current error log.
    pub fn log() -> Option<&'static [Error]> {
        unsafe {
            if let Some(errors) = &ERRORS {
                Some(errors)
            } else {
                None
            }
        }
    }

    /// Clears the current error log.
    pub fn clear() {
        unsafe {
            ERRORS = None;
        }
    }

    /// Construct a new error instance.
    pub fn new(message : &'static str, row : usize, column : usize) -> Error {
        Error {
            message,
            row,
            column
        }
    }

    /// Pushes this error onto the error list.
    pub fn throw(self) {
        unsafe {
            if let None = &ERRORS {
                ERRORS = Some(Vec::new());
            }
            match &mut ERRORS {
                Some(ref mut errors) => errors.push(self),
                _ => unreachable!()
            }
        }
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error at (row. {}, col. {}): {}",
                self.row, self.column, self.message)
    }
}