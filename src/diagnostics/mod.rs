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
    error_occured : Option<&'static str>,
    value : T
}
impl<T> DiagnosticBuilder<T> {
    /// Consumes the diagnostic and returns a boolean value depending on whether the value is valid.
    pub fn is_valid(self) -> bool {
        self.error_occured.is_none()
    }

    /// Consumes the diagnostic and adds some warning to the session if the diagnostic was invalid.
    pub fn warn(self, mut sess : Session, token : Token) -> Option<T> {
        if let Some(reason) = self.error_occured {
            sess.errors.push(Error { reason, token });
            None
        } else {
            Some(self.value)
        }
    }

    /// Consumes the diagnostic and adds some error to the session if the diagnostic was invalid.
    pub fn submit(self, mut sess : Session, token : Token) -> Option<T> {
        if let Some(reason) = self.error_occured {
            sess.errors.push(Error { reason, token });
            None
        } else {
            Some(self.value)
        }
    }
}
