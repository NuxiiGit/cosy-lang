use std::{ vec, slice, fmt, error };

/// Represents a source location.
pub type SourcePosition = usize;

/// Represents the differnt kinds of error.
#[derive(PartialOrd, PartialEq, Debug, Clone)]
pub enum ErrorKind {
    Warning,
    Bug,
    Fatal
}
impl Default for ErrorKind {
    fn default() -> Self {
        ErrorKind::Warning
    }
}

/// A struct which keeps track of errors.
#[derive(Default)]
pub struct IssueTracker {
    errors : Vec<Error>,
    pub error_level : ErrorKind
}
impl IssueTracker {
    /// Creates a new empty session.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns whether errors occurred.
    pub fn contains_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Adds a new error to the session.
    pub fn report(&mut self, error : Error) {
        if error.kind > self.error_level {
            self.error_level = error.kind.clone();
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
impl<'a> IntoIterator for &'a IssueTracker {
    type Item = &'a Error;
    type IntoIter = slice::Iter<'a, Error>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.errors).into_iter()
    }
}

/// Stores compile error information.
#[derive(Debug, Clone)]
pub struct Error {
    pub location : SourcePosition,
    pub kind : ErrorKind,
    pub reason : &'static str
}
impl Error {
    /// Creates a new fatal error
    pub fn warning(location : SourcePosition, reason : &'static str) -> Self {
        Self::new(location, ErrorKind::Warning, reason)
    }

    /// Creates a new fatal error
    pub fn bug(location : SourcePosition, reason : &'static str) -> Self {
        Self::new(location, ErrorKind::Bug, reason)
    }

    /// Creates a new fatal error
    pub fn fatal(location : SourcePosition, reason : &'static str) -> Self {
        Self::new(location, ErrorKind::Fatal, reason)
    }

    /// Creates a new error instance.
    pub fn new(location : SourcePosition, kind : ErrorKind, reason : &'static str) -> Self {
        Error { location, kind, reason }
    }
}
impl fmt::Display for Error {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "[byte. {}] {:?}! {}", self.location, self.kind, self.reason)
    }
}
impl error::Error for Error {}
