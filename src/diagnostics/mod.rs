pub mod error;

use error::{ Error, ErrorKind };

use std::vec;

/// A struct which keeps track of errors.
pub struct IssueTracker {
    errors : Vec<Error>,
    level : Option<ErrorKind>
}
impl IssueTracker {
    /// Creates a new empty session.
    pub fn new() -> Self {
        Self {
            errors : Vec::new(),
            level : None
        }
    }

    /// Returns the error level, if one exists.
    pub fn level(&self) -> Option<ErrorKind> {
        self.level.clone()
    }

    /// Adds a new error to the session.
    pub fn report(&mut self, error : Error) {
        let increase_level = if let Some(kind) = &self.level
                { error.kind > *kind } else { true };
        if increase_level {
            self.level = Some(error.kind.clone());
        }
        self.errors.push(error);
    }
}
impl IntoIterator for IssueTracker {
    type Item = Error;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.errors.into_iter()
    }
}