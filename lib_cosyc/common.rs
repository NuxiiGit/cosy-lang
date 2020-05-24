pub mod diagnostics;

use diagnostics::{ IssueTracker, SourcePosition };

/// A struct which stores session information, such as:
/// - Source code
/// - Errors
#[derive(Default)]
pub struct Session {
	/// The source code of the script you want o compile.
	pub src : String,
	/// Used to log any errors encountered during the session.
	pub issues : IssueTracker
}
impl Session {
	/// Produces a **sorted** list of source positions where a new line occurs.
	pub fn prospect_newlines(&self) -> Vec<SourcePosition> {
		let mut locations = Vec::new();
		let mut ignore_linefeed = false;
		for (i, x) in self.src.char_indices() {
			let contains_newline;
			if matches!(x, '\r') {
				contains_newline = true;
				ignore_linefeed = true;
			} else if matches!(x, '\n' if !ignore_linefeed) {
				contains_newline = true;
			} else {
				contains_newline = false;
				ignore_linefeed = false;
			}
			if contains_newline {
				locations.push(i);
			}
		}
		locations
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