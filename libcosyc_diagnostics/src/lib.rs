use libcosyc_source::Span;

use std::fmt;

/// Represents different kinds of error.
#[derive(PartialOrd, PartialEq, Debug, Clone)]
pub enum ErrorLevel {
    Warning,
    Bug,
    Fatal
}
impl Default for ErrorLevel {
    fn default() -> Self {
        Self::Warning
    }
}

#[derive(Debug, Clone)]
struct Error {
    pub span : Span,
    pub level : ErrorLevel,
    pub reason : String,
    pub notes : Vec<String>
}
impl fmt::Display for Error {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "{:?}! {}", self.level, self.reason)
    }
}

/// Records any issues that occurred, including the highest error level achieved.
#[derive(Default)]
pub struct IssueTracker {
    errors : Vec<Error>,
    /// The highest `ErrorLevel` registered by the issue tracker.
    pub error_level : ErrorLevel
}
impl IssueTracker {
    fn report(&mut self, error : Error) {
        if error.level > self.error_level {
            self.error_level = error.level.clone();
        }
        self.errors.push(error);
    }
}

/// Represents a diagnostic
#[derive(Default, Debug)]
pub struct Diagnostic {
    pub span : Span,
    pub error_level : ErrorLevel,
    pub reason : String,
    pub notes : Vec<String>
}
impl Diagnostic {
    /// Sets the error level of the diagnostic.
    pub fn level(mut self, level : ErrorLevel) -> Self {
        self.error_level = level;
        self
    }

    /// Adds a note to the diagnostic.
    pub fn note(mut self, note : String) -> Self {
        self.notes.push(note);
        self
    }

    /// Update the diagnostic reason.
    pub fn reason(mut self, reason : String) -> Self {
        self.reason = reason;
        self
    }

    /// Report the diagnostic to an issue tracker.
    pub fn report(self, issues : &mut IssueTracker) {
        issues.report(Error {
            span : self.span,
            level : self.error_level,
            reason : self.reason,
            notes : self.notes
        })
    }
}
impl<'a> From<&'a Span> for Diagnostic {
    fn from(span : &'a Span) -> Self {
        let mut diagnostic = Self::default();
        diagnostic.span.begin = span.begin;
        diagnostic.span.end = span.end;
        diagnostic
    }
}

/// Produces a **sorted** list of source positions where a new line occurs.
fn prospect_newlines(src : &str) -> Vec<Span> {
    let mut begin = 0;
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
        locations.push(Span { begin, end });
        begin = if let Some((i, _)) = chars.peek() { *i } else { src.len() };
    }
    locations.push(Span { begin, end : src.len() });
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

fn binary_search_newlines(lines : &[Span], pos : usize) -> Result<usize, usize> {
    lines.binary_search_by(|x| {
        use std::cmp::Ordering;
        if x.begin > pos {
            Ordering::Greater
        } else if x.end < pos {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    })
}

/// Represents a compiler session.
#[derive(Default)]
pub struct Session {
    /// The issue tracker that registers compiler errors.
    pub issues : IssueTracker,
    /// The filepath of the script to consider.
    pub filepath : String,
    /// The source of the script to consider.
    pub src : String
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
            let newlines = prospect_newlines(&self.src);
            for error in &self.issues.errors {
                let error_begin = error.span.begin;
                let error_end = error.span.end;
                let line_begin = binary_search_newlines(&newlines, error_begin).unwrap();
                let line_end = binary_search_newlines(&newlines, error_end).unwrap();
                let Span { begin : start, end } = newlines.get(line_begin).unwrap();
                let row = line_begin + 1;
                let col = error_begin - start + 1;
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

