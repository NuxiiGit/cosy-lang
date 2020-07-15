use std::{ vec, slice, fmt, error };

/// Represents a source location.
pub struct Span<T> {
    pub begin : usize,
    pub end : usize,
    pub content : T
}
impl<T> fmt::Display for Span<T> {
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
    pub level : ErrorLevel,
    pub reason : &'static str
}
impl fmt::Display for Error {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "{:?}! {}", self.level, self.reason)
    }
}

/// Represents a compiler session.
#[derive(Default)]
pub struct Session {
    errors : Vec<Span<Error>>,
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

    fn report(&mut self, error : Span<Error>) {
        if error.content.level > self.error_level {
            self.error_level = error.content.level.clone();
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
