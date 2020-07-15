use std::{ vec, slice, fmt, error };

/// Represents a source location.
#[derive(Default, Debug, Clone)]
pub struct Span {
    pub begin : usize,
    pub end : usize,
}
impl fmt::Display for Span {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "[{}..{}]", self.begin, self.end)
    }
}

/// Represents different kinds of error.
#[derive(PartialOrd, PartialEq, Debug, Clone)]
pub enum ErrorLevel {
    Warning,
    Bug,
    Fatal
}
impl Default for ErrorLevel {
    fn default() -> Self {
        Self::Warning
    }
}

#[derive(Debug, Clone)]
struct Error {
    pub span : Span,
    pub level : ErrorLevel,
    pub reason : String
}
impl fmt::Display for Error {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "{:?}! {}", self.level, self.reason)
    }
}

/// Represents a compiler session.
#[derive(Default)]
pub struct Session {
    errors : Vec<Error>,
    /// The highest `ErrorLevel` registered by the issue tracker.
    pub error_level : ErrorLevel,
    /// The filepath of the script to consider.
    pub filepath : String,
    /// The source of the script to consider.
    pub src : String
}
impl Session {
    /// Creates a new empty session.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns whether errors occurred.
    pub fn contains_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    fn report(&mut self, error : Error) {
        if error.level > self.error_level {
            self.error_level = error.level.clone();
        }
        self.errors.push(error);
    }
}
impl From<String> for Session {
    fn from(src : String) -> Self {
        let mut sess = Self::default();
        sess.src = src;
        sess
    }
}

/// Represents a diagnostic
#[derive(Default, Debug)]
pub struct Diagnostic {
    pub span : Span,
    pub error_level : ErrorLevel,
    pub reason : String
}
impl Diagnostic {

    /// Report the diagnostic to a session.
    pub fn report(self, sess : &mut Session) {
        sess.report(Error {
            span : self.span,
            level : self.error_level,
            reason : self.reason
        })
    }
}
impl From<Span> for Diagnostic {
    fn from(span : Span) -> Self {
        let mut diagnostic = Diagnostic::default();
        diagnostic.span = span;
        diagnostic
    }
}
