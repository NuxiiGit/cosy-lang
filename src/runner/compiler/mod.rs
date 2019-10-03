pub mod parser;
pub mod scanner;

use std::fmt;
use std::error;
use std::result;

/// An enum which describes compiler errors.
#[derive(Debug)]
pub enum Error {
    Only {
        description : &'static str,
        row : usize,
        column : usize
    },
    Many(Vec<Error>)
}
impl fmt::Display for Error {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Only { description, row, column } => {
                write!(f, "Compile error at (row. {}, col. {}): {}",
                        row, column, description)?;
            },
            Error::Many(errors) => {
                for e in errors {
                    write!(f, "{}\n", e)?;
                }
            }
        }
        Ok(())
    }
}
impl error::Error for Error {}

/// A custom result for compile errors.
pub type Result<T> = result::Result<T, Error>;