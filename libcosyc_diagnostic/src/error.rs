use crate::source::Span;

/// Represents different kinds of error.
#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Clone)]
pub enum ErrorLevel {
    Lint,
    Warning,
    Fatal,
    Bug
}

impl Default for ErrorLevel {
    fn default() -> Self {
        Self::Fatal
    }
}

/// Represents an error instance encountered by the compiler.
#[derive(Default, Debug)]
pub struct CompilerError {
    pub span : Option<Span>,
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
        self.span = Some(span.clone());
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

/// Records any issues that occurred, including the highest error level achieved.
#[derive(Default)]
pub struct IssueTracker {
    errors : Vec<CompilerError>,
    error_level : ErrorLevel
}

impl IssueTracker {
    /// Reports an error to the issue tracker.
    pub fn report_error(&mut self, error : CompilerError) {
        if error.level > self.error_level {
            self.error_level = error.level.clone();
        }
        self.errors.push(error);
    }

    /// Returns a reference to the current error level of the issue tracker.
    pub fn get_severity(&self) -> &ErrorLevel {
        &self.error_level
    }

    /// Returns a reference to the list of compiler errors.
    pub fn get_errors(&self) -> &[CompilerError] {
        &self.errors
    }
}
