pub mod source;
pub mod error;

use error::{ ErrorLevel, CompilerError };

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

