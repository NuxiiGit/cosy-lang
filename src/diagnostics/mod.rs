pub mod error;

use crate::syntax::token::Token;

use error::Error;

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

/// Builds a diagnostic which can be used to throw errors or check validity.
pub struct DiagnosticBuilder<T> {
    error_occured : bool,
    value : T
}
impl<T> DiagnosticBuilder<T> {
    /// Creates a new diagnostic builder with this value.
    pub fn from(value : T) -> Self {
        Self {
            error_occured : false,
            value
        }
    }

    /// Consumes the diagnostic and returns a boolean value depending on whether the value is valid.
    pub fn check(self) -> bool {
        self.error_occured
    }

    /// Reports a warning to the session if the value was invalid according to the current diagnostic.
    pub fn warn(self, mut sess : Session, error : Error) -> Self {
        if self.error_occured {
            sess.warnings.push(error);
        }
        self
    }
    
    /// Reports a fatal error to the session if the value was invalid according to the current diagnostic.
    pub fn error(self, mut sess : Session, error : Error) -> Self {
        if self.error_occured {
            sess.warnings.push(error);
        }
        self
    }
}
