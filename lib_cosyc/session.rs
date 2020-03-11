use crate::diagnostics::IssueTracker;
use crate::span::Span;

use std::str::CharIndices;
use std::mem;

/// A struct which stores session information, such as:
/// - Source code
/// - Character stream
/// - Errors
pub struct Session<'a> {
    /// The source code of the script you want o compile.
    pub src : &'a str,
    /// Used to log any errors encountered during the session.
    pub issues : IssueTracker
}
impl<'a> Session<'a> {
    /// Creates a new parser session from this source code.
    pub fn from(src : &'a str) -> Self {
        Self {
            src,
            issues : IssueTracker::new()
        }
    }
}