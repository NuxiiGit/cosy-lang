pub mod diagnostics;

use diagnostics::IssueTracker;

use std::fmt;

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
impl fmt::Display for Session {
	fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
		let newlines = prospect_newlines(&self.src);
		for issue in &self.issues {
			writeln!(out, "{:?}: {}", issue.kind, issue.reason)?;
		}
		Ok(())
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

/// Produces a **sorted** list of source positions where a new line occurs.
pub fn prospect_newlines(src : &str) -> Vec<usize> {
	let mut locations = Vec::new();
	let mut ignore_linefeed = false;
	for (i, x) in src.char_indices() {
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

/// Uses a binary search to locate the row and column number of this source location.
pub fn infer_source_location(lines : &[usize], index : usize) -> (usize, usize) {
	unimplemented!()
}