use crate::source::Span;

use std::fmt;

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
pub (crate) struct Error {
    pub span : Span,
    pub level : ErrorLevel,
    pub reason : String,
    pub notes : Vec<String>
}
impl fmt::Display for Error {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "{:?}! {}", self.level, self.reason)
    }
}

/// Records any issues that occurred, including the highest error level achieved.
#[derive(Default)]
pub struct IssueTracker {
    pub (crate) errors : Vec<Error>,
    /// The highest `ErrorLevel` registered by the issue tracker.
    pub error_level : ErrorLevel
}
impl IssueTracker {
    fn report(&mut self, error : Error) {
        if error.level > self.error_level {
            self.error_level = error.level.clone();
        }
        self.errors.push(error);
    }
}

/// Represents a diagnostic
#[derive(Default, Debug)]
pub struct Diagnostic {
    pub span : Span,
    pub error_level : ErrorLevel,
    pub reason : String,
    pub notes : Vec<String>
}
impl Diagnostic {
    /// Sets the error level of the diagnostic.
    pub fn level(mut self, level : ErrorLevel) -> Self {
        self.error_level = level;
        self
    }

    /// Adds a note to the diagnostic.
    pub fn note(mut self, note : String) -> Self {
        self.notes.push(note);
        self
    }

    /// Similar to `note`, except a borrowed value is used.
    pub fn note_str(mut self, note : &str) -> Self {
        self.note(note.to_string())
    }

    /// Update the diagnostic reason.
    pub fn reason(mut self, reason : String) -> Self {
        self.reason = reason;
        self
    }

    /// Similar to `reason`, except a borrowed value is used.
    pub fn reason_str(mut self, reason : &str) -> Self {
        self.reason(reason.to_string())
    }

    /// Report the diagnostic to an issue tracker.
    pub fn report(self, issues : &mut IssueTracker) {
        issues.report(Error {
            span : self.span,
            level : self.error_level,
            reason : self.reason,
            notes : self.notes
        })
    }
}
impl<'a> From<&'a Span> for Diagnostic {
    fn from(span : &'a Span) -> Self {
        let mut diagnostic = Self::default();
        diagnostic.span.begin = span.begin;
        diagnostic.span.end = span.end;
        diagnostic
    }
}
