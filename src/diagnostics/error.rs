use crate::syntax::token::Token;

use std::fmt;
use std::error;
use std::iter::IntoIterator;

/// A struct which stores compiler session information.
pub struct Session {
    errors : Vec<Error>
}
impl Session {
    /// Creates a new empty session.
    pub fn new() -> Self {
        Self {
            errors : Vec::new()
        }
    }

    /// Returns whether the session is empty.
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Adds a new error to the session.
    pub fn report(&mut self, error : Error) {
        self.errors.push(error);
    }
}
impl IntoIterator for Session {
    type Item = Error;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.errors.into_iter()
    }
}

/// A struct which stores error information.
#[derive(Debug)]
pub struct Error {
    pub reason : &'static str,
    pub token : Token
}
impl fmt::Display for Error {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "{}: {}. got {:?}",
                self.token.span, self.reason, self.token.kind)
    }
}
impl error::Error for Error {}