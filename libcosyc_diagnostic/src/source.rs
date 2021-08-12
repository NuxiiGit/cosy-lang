use std::{ cmp, fmt };

/// Represents the span of bytes of a substring within a source file.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Span {
    /// The starting byte of the span.
    pub begin : usize,
    /// The ending byte of the span.
    pub end : usize
}

impl Span {
    /// Creates a new span from these source positions.
    pub fn new(begin : usize, end : usize) -> Self {
        Self { begin, end }
    }

    /// Renders a substring of a string slice using this byte span.
    pub fn render<'a>(&self, src : &'a str) -> &'a str {
        &src[self.begin..self.end]
    }

    /// Joins two spans together using the largest range between them.
    pub fn join(&self, other : &Self) -> Self {
        let min = cmp::min(self.begin, other.begin);
        let max = cmp::max(self.end, other.end);
        Self::new(min, max)
    }

    /// Returns whether the starting byte of the span is greater than or equal to its ending byte.
    pub fn is_degenerate(&self) -> bool {
        self.begin >= self.end
    }
}

impl fmt::Display for Span {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "[{}..{}]", self.begin, self.end)
    }
}

/// Produces a sorted list of source positions where a new line occurs.
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
        locations.push(Span::new(begin, end));
        begin = if let Some((i, _)) = chars.peek() {
            *i
        } else {
            src.len()
        };
    }
    locations.push(Span::new(begin, src.len()));
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

/// Supplies a trait that helps structs render spans from a piece of source code.
pub trait Renderable {
    /// Exposes the source code of the implementing struct.
    fn src(&self) -> &str;
    /// Renders this span using the content from the source file.
    fn render(&self, span : &Span) -> &str {
        span.render(&self.src())
    }
}
