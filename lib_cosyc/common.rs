pub mod diagnostics;

use diagnostics::IssueTracker;

use std::{ fmt, fs, io };

/// A struct which stores session information, such as:
/// - Source code
/// - Errors
pub struct Session {
	/// The file location of the source code (if it exists).
	pub filepath : String,
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
			filepath : String::from(filepath),
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
			write!(out, " --> ")?;
			write!(out, "{}:", self.filepath)?;
			writeln!(out, "[row. {}, col. {}]", row, col)?;
		}
		Ok(())
	}
}
impl From<String> for Session {
	fn from(src : String) -> Self {
		Self {
			filepath : String::new(),
			src,
			issues : IssueTracker::new()
		}
	}
}

/// Produces a **sorted** list of source positions where a new line occurs.
pub fn prospect_newlines(src : &str) -> Vec<usize> {
	let mut locations = Vec::new();
	let mut chars = src.char_indices().peekable();
	while let Some((_, next)) = chars.next() {
		match next {
			'\r' if matches!(chars.peek(), Some((_, '\n'))) => {
				chars.next();
			},
			'\r' | '\n' => (),
			_ => continue
		}
		let location = if let Some((i, _)) = chars.peek() { *i } else { src.len() };
		locations.push(location);
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
	let row = line_no;
	let col = index - line_byte;
	(row + 1, col + 1)
}