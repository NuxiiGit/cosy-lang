use std::fmt;

/// Represents a source location.
#[derive(Default, Debug, Clone)]
pub struct Span {
    pub begin : usize,
    pub end : usize,
}
impl fmt::Display for Span {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "[{}..{}]", self.begin, self.end)
    }
}

/// Produces a **sorted** list of source positions where a new line occurs.
pub fn prospect_newlines(src : &str) -> Vec<Span> {
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

/// Searches an array of newline spans for a specific byte position, `pos`.
pub fn binary_search_newlines(lines : &[Span], pos : usize) -> Result<usize, usize> {
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
