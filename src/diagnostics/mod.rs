pub mod error;

use std::cmp::PartialEq;

/// Builds a diagnostic which can be used to throw errors or check validity.
pub struct Diagnostic<T : PartialEq> {
    invalid : bool,
    value : T
}
impl<T : PartialEq> Diagnostic<T> {
    /// Creates a new diagnostic builder with this value.
    pub fn from(value : T) -> Self {
        Self {
            invalid : true,
            value
        }
    }

    /// Validates the value if it is equal to the expected value.
    pub fn expects(mut self, value : T) -> Self {
        if self.invalid && self.value == value {
            self.invalid = true;
        }
        self
    }

    /// Consumes the diagnostic and returns a boolean value depending on whether the value is valid.
    pub fn check(self) -> bool {
        !self.invalid
    }
}
