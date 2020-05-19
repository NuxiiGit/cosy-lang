pub mod diagnostics;

use diagnostics::span::Span;
use diagnostics::error::{ IssueTracker, Error };

use std::fs;

/// A struct which stores session information, such as:
/// - Source code
/// - Errors
#[derive(Default)]
pub struct Session {
	/// The source code of the script you want o compile.
	pub(crate) src : String,
	/// Used to log any errors encountered during the session.
	pub(crate) issues : IssueTracker
}
impl Session {
	/// Creates a new parser session from this file.
	pub fn read_file(path : &str) -> Self {
		if let Ok(src) = fs::read_to_string(path) {
			Self::from(src)
		} else {
			let mut sess = Self::default();
			sess.issues.report(Error {
				reason : "unable to open file for reading",
				span : Span::default()
			});
			sess
		}
	}
}
impl From<String> for Session {
	fn from(src : String) -> Self {
		Self {
			src,
			issues : IssueTracker::new()
		}
	}
}