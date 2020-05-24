pub mod diagnostics;

use diagnostics::IssueTracker;

use std::{ fmt, fs, io };

/// A struct which stores session information, such as:
/// - Source code
/// - Errors
pub struct Session {
	/// The file location of the source code (if it exists).
	pub filepath : Option<String>,
	/// The source code of the script you want o compile.
	pub src : String,
	/// Used to log any errors encountered during the session.
	pub issues : IssueTracker
}
impl Session {
	/// Reads a source file and produces a parser session.
	pub fn read(filepath : &str) -> io::Result<Session> {
		let src = fs::read_to_string(filepath)?;
		Ok(Self {
			filepath : Some(String::from(filepath)),
			src,
			issues : IssueTracker::new()
		})
	}
}
impl fmt::Display for Session {
	fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
		let newlines = prospect_newlines(&self.src);
		for issue in &self.issues {
			let (row, col) = infer_source_location(&newlines, issue.location);
			writeln!(out, "{:?}: {}", issue.kind, issue.reason)?;
			writeln!(out, " --> [row. {}, col. {}]", row, col)?;
		}
		Ok(())
	}
}
impl From<String> for Session {
	fn from(src : String) -> Self {
		Self {
			filepath : None,
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
	let mut line_no = 0;
	let mut line_byte = 0;
	// linear search temporarily
	for &i in lines {
		if index < i {
			break;
		}
		line_no += 1;
		line_byte = i;
	}
	let row = line_no + 1;
	let col = index - line_byte;
	(row, col)
}