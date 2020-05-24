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
			let indent = " ".repeat(digit_count(row));
			writeln!(out, "{:?}: {}", issue.kind, issue.reason)?;
			write!(out, " {}--> ", indent)?;
			write!(out, "{}:", self.filepath)?;
			writeln!(out, "[row. {}, col. {}]", row, col)?;
			writeln!(out, " {} | ", indent)?;
			writeln!(out, " {} | {}", row, &self.src[*start..*end].replace("\t", " "))?;
			writeln!(out, " {} |{}^", indent, " ".repeat(col))?;
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

/// Returns the number of digits of this natural number.
fn digit_count(mut n : usize) -> usize {
	let mut count = 1;
	loop {
		if n < 10 {
			return count;
		} else {
			n /= 10;
			count += 1;
		}
	}
}