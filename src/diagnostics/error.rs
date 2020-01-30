use crate::syntax::token::Token;

use std::fmt;
use std::error;

/// A struct which stores compiler session information.
pub struct Session {
    fatal_occurred : bool,
    errors : Vec<Error>
}
impl Session {
    /// Creates a new empty session.
    pub fn new() -> Self {
        Self {
            fatal_occurred : false,
            errors : Vec::new()
        }
    }

    /// Adds a new error to the session.
    pub fn fatal(&mut self, error : Error) {
        self.errors.push(error);
        self.fatal_occurred = true;
    }

    /// Adds a new warning to the session.
    pub fn warning(&mut self, error : Error) {
        self.errors.push(error);
    }

    /// Returns whether a fatal error occured.
    pub fn is_fatal(&self) -> bool {
        self.fatal_occurred
    }

    /// Returns a reference to all errors.
    pub fn errors(&self) -> &[Error] {
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