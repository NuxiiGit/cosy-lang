pub mod source;
pub mod error;

use source::Span;
use error::IssueTracker;

use std::fmt;

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

/// Represents a compiler session.
#[derive(Default)]
pub struct Session {
    /// The issue tracker that registers compiler errors.
    pub issues : IssueTracker,
    /// The filepath of the script to consider.
    pub filepath : String,
    /// The source of the script to consider.
    pub src : String,
    /// The output stream.
    pub out : String
}
impl Session {
    /// Creates a new empty session.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns whether errors occurred in the current session.
    pub fn contains_errors(&self) -> bool {
        !self.issues.errors.is_empty()
    }
}
impl From<String> for Session {
    fn from(src : String) -> Self {
        let mut sess = Self::default();
        sess.src = src;
        sess
    }
}
impl fmt::Display for Session {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        // it works, i don't care if it's trash
        if self.contains_errors() {
            let newlines = source::prospect_newlines(&self.src);
            for error in &self.issues.errors {
                let error_begin = error.span.begin;
                let error_end = error.span.end;
                let line_begin = source::binary_search_newlines(&newlines, error_begin).unwrap();
                let line_end = source::binary_search_newlines(&newlines, error_end).unwrap();
                let Span { begin : start, end } = newlines.get(line_begin).unwrap();
                let Span { begin : start_end, end : _ } = newlines.get(line_end).unwrap();
                let row = line_begin + 1;
                let col = error_begin - start + 1;
                let col_end = error_end - start_end + 1;
                let indent_length = digit_count(line_end + 1);
                let indent = " ".repeat(indent_length);
                writeln!(out, "")?;
                writeln!(out, "{:?}: {}", error.level, error.reason)?;
                write!(out, " {}>>> ", indent)?;
                write!(out, "{}@", self.filepath)?;
                writeln!(out, "[row. {}, col. {}]", row, col)?;
                writeln!(out, " {} | ", indent)?;
                if line_begin == line_end {
                    // underline error
                    let mut underline_length = error_end - error_begin;
                    if underline_length < 1 {
                        underline_length = 1;
                    }
                    writeln!(out, " {:width$} | {}", row, &self.src[*start..*end].replace("\t", " "), width=indent_length)?;
                    writeln!(out, " {} |{}{}", indent, " ".repeat(col), "^".repeat(underline_length))?;
                } else {
                    // display lines of error
                    writeln!(out, " {} |{}{}", indent, " ".repeat(col), " starts here")?;
                    writeln!(out, " {} |{}{}", indent, " ".repeat(col), "/")?;
                    for line in line_begin..=line_end {
                        if line > line_begin + 1 {
                            if line < line_end - 2 {
                                continue;
                            } else if line < line_end - 1 {
                                writeln!(out, " {}...", indent)?;
                                continue;
                            }
                        }
                        let Span { begin : start, end } = newlines.get(line).unwrap();
                        writeln!(out, " {:width$} | {}", line + 1, &self.src[*start..*end].replace("\t", " "), width=indent_length)?;
                    }
                    writeln!(out, " {} |{}{}", indent, " ".repeat(col_end), "\\")?;
                    writeln!(out, " {} |{}{}", indent, " ".repeat(col_end), " ends here")?;
                }
                if !error.notes.is_empty() {
                    // display notes
                    writeln!(out, " {} | ", indent)?;
                    for note in &error.notes {
                        writeln!(out, " {} ? Note: {}", indent, note)?;
                    }
                }
            }
            Ok(())
        } else {
            write!(out, "no errors occurred")
        }
    }
}

