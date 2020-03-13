use crate::diagnostics::*;
use crate::span::*;
use crate::token::TokenKind;

use std::fs;
use std::collections::VecDeque;

/// A struct which stores session information, such as:
/// - Source code
/// - Character stream
/// - Errors
pub struct Session {
    /// The source code of the script you want o compile.
    pub src : String,
    /// Stores tokens and their position in the source file.
    pub tokens : VecDeque<Context<TokenKind>>,
    /// Used to log any errors encountered during the session.
    pub issues : IssueTracker
}
impl Session {
    /// Creates a new parser session from this source code.
    pub fn from(src : String) -> Self {
        Self {
            src,
            tokens : VecDeque::new(),
            issues : IssueTracker::new()
        }
    }

    /// Creates a new parser session from this file.
    pub fn read(path : &str) -> Self {
        if let Ok(src) = fs::read_to_string(path) {
            Self::from(src)
        } else {
            let mut sess = Self::from(String::new());
            sess.issues.report(Error {
                reason : "unable to open file for reading",
                kind : ErrorKind::Fatal,
                span : Span::new()
            });
            sess
        }
    }
}