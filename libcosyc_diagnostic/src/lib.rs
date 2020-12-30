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
    pub fn report(&mut self, error : CompilerError) {
        if error.level > self.error_level {
            self.error_level = error.level.clone();
        }
        self.errors.push(error);
    }
}

