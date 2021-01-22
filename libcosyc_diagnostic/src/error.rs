use crate::source::Span;

/// Represents different kinds of error.
#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Clone)]
pub enum ErrorLevel {
    Warning,
    Fatal
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

    /// Update the error reason.
    pub fn reason<T : ToString>(mut self, reason : T) -> Self {
        self.reason = reason.to_string();
        self
    }

    /// Adds a note to the error.
    pub fn note<T : ToString>(mut self, note : T) -> Self {
        self.notes.push(note.to_string());
        self
    }

    /// Returns whether this error has a span.
    pub fn has_span(&self) -> bool {
        self.span.is_some()
    }

    /// Returns a standard linting error.
    pub fn warning() -> Self {
        CompilerError::new().level(ErrorLevel::Warning)
    }

    /// Returns a standard bug error.
    pub fn bug() -> Self {
        CompilerError::new()
                .level(ErrorLevel::Fatal)
                .note("this may be caused by a bug in the compiler internals")
    }

    /// Returns a built-in error for unimplemented features.
    pub fn unimplemented<T : ToString>(reason : T) -> Self {
        CompilerError::new()
                .reason(format!("{} is not currently supported", reason.to_string()))
    }

    /// Returns a built-in error for unstable features.
    pub fn unstable<T : ToString>(reason : T) -> Self {
        CompilerError::warning()
                .reason(format!("{} is currently unstable", reason.to_string()))
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

/// Supplies a trait that helps structs report errors to an issue tracker.
pub trait Failable {
    /// Exposes the issue tracker of the implementing struct.
    fn issues(&mut self) -> &mut IssueTracker;
    /// Reports an error to the issue tracker and returns `None`.
    fn report<T>(&mut self, error : CompilerError) -> Option<T> {
        self.issues().report_error(error);
        None
    }
    /// Defined in terms of `report<T>` where `T` = empty type.
    fn report_empty(&mut self, error : CompilerError) -> Option<()> {
        self.report(error)
    }
}
