use crate::source::Span;
use std::fmt;

/// Represents different kinds of error.
#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Clone)]
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

/// Represents an error instance encountered by the compiler.
#[derive(Default, Debug)]
pub struct CompilerError {
    pub span : Span,
    pub level : ErrorLevel,
    pub reason : String,
    pub notes : Vec<String>
}

impl CompilerError {
    /// Creates a new empty error.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the span of the error.
    pub fn span(mut self, span : &Span) -> Self {
        self.span = span.clone();
        self
    }

    /// Sets the severity of the error.
    pub fn level(mut self, level : ErrorLevel) -> Self {
        self.level = level;
        self
    }

    /// Adds a note to the error.
    pub fn note<T : ToString>(mut self, note : T) -> Self {
        self.notes.push(note.to_string());
        self
    }

    /// Update the error reason.
    pub fn reason<T : ToString>(mut self, reason : T) -> Self {
        self.reason = reason.to_string();
        self
    }
}

impl fmt::Display for CompilerError {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "{:?}! {}", self.level, self.reason)
    }
}
