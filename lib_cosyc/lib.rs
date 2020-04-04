pub mod span;
pub mod issues;
pub mod name_table;
pub mod lexer;

use issues::IssueTracker;
use name_table::NameTable;

/// A struct which stores session information, such as:
/// - Source code
/// - Errors
pub struct Session<'a> {
    /// The source code of the script you want to compile.
    pub(crate) src : &'a str,
    /// The name table for identifiers.
    pub(crate) name_table : NameTable<'a>,
    /// Used to log any errors encountered during the session.
    pub(crate) issues : IssueTracker
}
impl<'a> Session<'a> {
    /// Creates a new parser session from this source code.
    pub fn from(src : &'a str) -> Self {
        Self {
            src,
            name_table : NameTable::new(),
            issues : IssueTracker::new()
        }
    }
}