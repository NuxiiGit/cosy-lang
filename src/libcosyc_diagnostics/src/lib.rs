use libcosyc_common::source::SourcePos;

use std::{ fmt, error };
use std::vec;

/// A struct which keeps track of errors.
pub struct IssueTracker<'a> {
    errors : Vec<Error<'a>>,
    level : Option<ErrorKind>
}
impl<'a> IssueTracker<'a> {
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
    pub fn report(&mut self, error : Error<'a>) {
        let increase_level = if let Some(kind) = &self.level
                { error.kind > *kind } else { true };
        if increase_level {
            self.level = Some(error.kind.clone());
        }
        self.errors.push(error);
    }
}
impl<'a> IntoIterator for IssueTracker<'a> {
    type Item = Error<'a>;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.errors.into_iter()
    }
}

/// A struct which stores error information.
#[derive(Debug)]
pub struct Error<'a> {
    pub reason : &'static str,
    pub src_pos : SourcePos<'a>,
    pub kind : ErrorKind
}
impl fmt::Display for Error<'_> {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "{:?}! line {}: {}",
                self.kind, self.src_pos.line, self.reason)
    }
}
impl error::Error for Error<'_> {}

/// An enum which describes available error types.
#[derive(PartialOrd, PartialEq, Debug, Clone)]
pub enum ErrorKind {
    Warning,
    Fatal
}