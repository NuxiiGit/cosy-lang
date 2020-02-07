pub mod error;

use error::Error;

use std::vec;

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
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.errors.into_iter()
    }
}