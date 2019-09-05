#![allow(dead_code)]

use std::fmt;

static mut ERRORS : Option<Vec<Error>> = None;

/// A struct which holds error information.
pub struct Error {
    message : &'static str,
    row : usize,
    column : usize
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
    pub fn throw(message : &'static str, row : usize, column : usize) {
        unsafe {
            if let None = &ERRORS {
                ERRORS = Some(Vec::new());
            }
            match &mut ERRORS {
                Some(ref mut errors) => {
                    errors.push(Error {
                        message : message,
                        row : row,
                        column : column
                    });
                },
                _ => unreachable!()
            }
        }
    }

    /// Returns the error message.
    pub fn message(&self) -> &'static str {
        self.message
    }

    /// Returns the row number the error occured on.
    pub fn row(&self) -> usize {
        self.row
    }

    /// Returns the column number the error occured on.
    pub fn column(&self) -> usize {
        self.column
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error at (row. {}, col. {}): {}",
                self.row, self.column, self.message)
    }
}