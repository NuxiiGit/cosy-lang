pub mod diagnostics;

use diagnostics::{ IssueTracker };

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
			let location = issue.location;
			let line = newlines.binary_search_by(|x| {
				use std::cmp::Ordering;
				if x.0 > location {
					Ordering::Greater
				} else if x.1 < location {
					Ordering::Less
				} else {
					Ordering::Equal
				}
			}).unwrap();
			let (start, end) = newlines.get(line).unwrap();
			let row = line + 1;
			let col = location - start + 1;
			writeln!(out, "{:?}: {}", issue.kind, issue.reason)?;
			write!(out, " --> ")?;
			write!(out, "{}:", self.filepath)?;
			writeln!(out, "[row. {}, col. {}]", row, col)?;
			writeln!(out, "  | ")?;
			writeln!(out, "  | {}", &self.src[*start..*end])?;
			writeln!(out, "  |{}^", " ".repeat(col))?;
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
fn prospect_newlines(src : &str) -> Vec<(usize, usize)> {
	let mut start = 0;
	let mut locations = Vec::new();
	let mut chars = src.char_indices().peekable();
	while let Some((end, next)) = chars.next() {
		match next {
			'\r' if matches!(chars.peek(), Some((_, '\n'))) => {
				chars.next();
			},
			'\r' | '\n' => (),
			_ => continue
		}
		locations.push((start, end));
		start = if let Some((i, _)) = chars.peek() { *i } else { src.len() };
	}
	locations.push((start, src.len()));
	locations
}