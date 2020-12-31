use std::{ fmt, cmp };

/// Represents a source location.
#[derive(Default, Debug, Clone)]
pub struct Span {
    /// The start byte of the span.
    pub begin : usize,
    /// The end byte of the span.
    pub end : usize
}

impl Span {
    /// Creates a new span from these source positions.
    pub fn new(begin : usize, end : usize) -> Self {
        Self { begin, end }
    }

    /// Renders a substring using this span.
    pub fn render<'s>(&self, src : &'s str) -> &'s str {
        &src[self.begin..self.end]
    }

    /// Joins two spans together using their largest range.
    pub fn join(&self, other : &Self) -> Self {
        let min = cmp::min(self.begin, other.begin);
        let max = cmp::max(self.end, other.end);
        Self::new(min, max)
    }

    /// Returns whether the span is degenerate.
    pub fn is_degenerate(&self) -> bool {
        self.begin == self.end
    }
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
        if let '\r' | '\n' = next {
            if next == '\r' && matches!(chars.peek(), Some((_, '\n'))) {
                chars.next();
            }
        } else {
            continue;
        }
        locations.push(Span { begin, end });
        begin = if let Some((i, _)) = chars.peek() {
            *i
        } else {
            src.len()
        };
    }
    locations.push(Span { begin, end : src.len() });
    locations
}

/// Searches an array of newline spans for a specific byte position, `pos`.
pub fn binary_search_newlines(lines : &[Span], pos : usize) -> Result<usize, usize> {
    lines.binary_search_by(|x| {
        if x.begin > pos {
            cmp::Ordering::Greater
        } else if x.end < pos {
            cmp::Ordering::Less
        } else {
            cmp::Ordering::Equal
        }
    })
}
