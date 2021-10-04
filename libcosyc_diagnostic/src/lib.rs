pub mod source;
pub mod error;

use error::{ IssueTracker, CompilerError };
use source::Span;
use std::{ fmt, fs };

/// Holds a reference to the session data, to be passed to data structures and modified.
pub struct SessionData<'a> {
    /// A reference to the sessions issue tracker.
    pub issues : &'a mut IssueTracker,
    /// A reference to the source code of the session.
    pub src : &'a str,
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
}

impl Session {
    /// Creates a new session from this string.
    pub fn new(src : &str) -> Self {
        let mut sess = Self::default();
        sess.src = src.to_string();
        sess
    }

    /// Returns a reference to the session data.
    pub fn borrow_data(&mut self) -> SessionData {
        let issues = &mut self.issues;
        let src = &self.src as &str;
        SessionData { issues, src }
    }

    /// Creates a new session using this file path.
    pub fn load(path : &str) -> Self {
        let mut sess = Self::default();
        sess.filepath = path.to_string();
        if let Ok(src) = fs::read_to_string(&path) {
            sess.src = src;
        } else {
            sess.issues.report_error::<()>(CompilerError::new()
                    .reason(format!("unable to open a file with the name `{}`", path))
                    .note("check the filename is correct"));
        }
        sess
    }

    /// Returns whether errors occurred in the current session.
    pub fn errors_occurred(&self) -> bool {
        !self.issues.get_errors().is_empty()
    }
}

impl fmt::Display for Session {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        // it works, i don't care if it's trash
        if self.errors_occurred() {
            let newlines = source::prospect_newlines(&self.src);
            for error in self.issues.get_errors() {
                let level_str = format!("{:?}", error.level);
                let note_str = "Note".to_string();
                let level_str_length = level_str.len();
                let note_str_length = note_str.len();
                let (level_indent_length, note_indent_length) = if level_str_length > note_str_length {
                    (0, level_str_length - note_str_length)
                } else {
                    (note_str_length - level_str_length, 0)
                };
                let level_indent = " ".repeat(level_indent_length);
                let note_indent = " ".repeat(note_indent_length);
                writeln!(out, "\n{}{}: {}", level_indent, level_str, error.reason)?;
                for note in &error.notes {
                    writeln!(out, "{}Note? {}", note_indent, note)?;
                }
                if let Some(span) = &error.span {
                    let error_begin = span.begin;
                    let error_end = span.end;
                    let line_begin = source::binary_search_newlines(&newlines, error_begin).unwrap();
                    let line_end = source::binary_search_newlines(&newlines, error_end).unwrap();
                    let Span { begin : start, end } = newlines.get(line_begin).unwrap();
                    let Span { begin : start_end, end : _ } = newlines.get(line_end).unwrap();
                    let row = line_begin + 1;
                    let col = error_begin - start + 1;
                    let col_end = error_end - start_end + 1;
                    let mut indent_length = (((line_end + 1) as f64).log10() + 1.0).floor() as usize;
                    if indent_length == 0 {
                        indent_length = 1;
                    }
                    let indent = " ".repeat(indent_length);
                    if !self.filepath.is_empty() {
                        writeln!(out, " {}>>> {}:{}:{}", indent, self.filepath, row, col)?;
                    }
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
                }
            }
            Ok(())
        } else {
            write!(out, "no errors occurred")
        }
    }
}

