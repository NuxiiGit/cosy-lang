use crate::syntax::token::Token;

use std::fmt;
use std::error;

/// A struct which stores compiler session information.
pub struct Session {
    errors : Vec<Error>,
    warnings : Vec<Error>
}
impl Session {
    /// Creates a new empty session.
    pub fn new() -> Self {
        Self {
            errors : Vec::new(),
            warnings : Vec::new()
        }
    }

    /// Adds a new error to the session.
    pub fn add_error(&mut self, error : Error) {
        self.errors.push(error);
    }

    /// Adds a new warning to the session.
    pub fn add_warning(&mut self, error : Error) {
        self.warnings.push(error);
    }

    /// Returns whether a fatal error occured.
    pub fn is_fatal(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Returns a reference to all fatal errors.
    pub fn errors(&self) -> &[Error] {
        &self.errors
    }

    /// Returns a reference to all warnings.
    pub fn warnings(&self) -> &[Error] {
        &self.warnings
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