use crate::span::Span;

use std::{ fmt, error };
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

/// A struct which stores error information.
#[derive(Debug)]
pub struct Error {
    pub reason : &'static str,
    pub kind : ErrorKind,
    pub span : Span
}
impl fmt::Display for Error {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "{:?}! {}: {}",
                self.kind, self.span, self.reason)
    }
}
impl error::Error for Error {}

/// An enum which describes available error types.
#[derive(PartialOrd, PartialEq, Debug, Clone)]
pub enum ErrorKind {
    Warning,
    Fatal
}