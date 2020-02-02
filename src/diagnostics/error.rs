use crate::syntax::token::Token;

use std::fmt;
use std::error;
use std::ops::Deref;

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

    /// Adds a new error to the session.
    pub fn report(&mut self, error : Error) {
        self.errors.push(error);
    }

    /// Returns a reference to all errors.
    pub fn errors(&self) -> &[Error] {
        &self.errors
    }
}
impl Deref for Session {
    type Target = Vec<Error>;

    fn deref(&self) -> &Self::Target {
        &self.errors
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