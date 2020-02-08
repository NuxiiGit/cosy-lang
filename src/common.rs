use crate::diagnostics::IssueTracker;
use crate::syntax::token::Token;

/// A struct which stores information about the compiler session.
pub struct Session {
    pub issues : IssueTracker,
    pub tokens : Vec<Token>
}
impl Session {
    /// Creates a new parser session.
    pub fn new() -> Self {
        Self {
            issues : IssueTracker::new(),
            tokens : Vec::new()
        }
    }
}