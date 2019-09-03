#![allow(dead_code)]

/// A struct which allows you to iterate and slice `&str` types.
pub struct Scanner<'a> {
    context : &'a str,
    chars : Chars<'a>,
    current : Option<char>,
    start : usize,
    end : usize,
    row : usize,
    col : usize
}
impl<'a> Scanner<'a> {
    /// Construct a new scanner.
    pub fn new(context : &str) -> Scanner {
        let mut scanner : Scanner = Scanner {
            context : context,
            chars : context.char_indices(),
            current : None,
            start : 0,
            end : 0,
            row : 0,
            col : 0,
        };
        scanner.next();
        scanner
    }

    /// Peek at the next character.
    pub fn peek(&self) -> Option<&char> {
        if let Some(ref x) = self.current {
            Some(x)
        } else {
            None
        }
    }

    /// Move to the next character.
    pub fn next(&mut self) -> Option<char> {
        let current : Option<char> = self.current;
        if let Some(x) = current {
            if let '\n' = x {
                self.row += 1;
                self.col = 0;
            } else {
                self.col += 1;
            }
        }
        if let Some((i, x)) = self.chars.next() {
            self.current = Some(x);
            self.end = i;
        } else {
            self.current = None;
            self.end = self.context.len();
        }
        current
    }

    /// Munches the current substring and returns its slice.
    pub fn munch(&mut self) -> &'a str {
        let slice : &str = self.slice();
        self.drop();
        slice
    }

    /// Returns the current substring.
    pub fn slice(&self) -> &'a str {
        &self.context[self.start..self.end]
    }

    /// Returns the left partition at that index.
    pub fn slice_left(&self, index : usize) -> &'a str {
        &self.context[self.start..index]
    }

    /// Returns the left partition at that index.
    pub fn slice_right(&self, index : usize) -> &'a str {
        &self.context[index..self.end]
    }

    /// Drops the current substring.
    pub fn drop(&mut self) {
        self.start = self.end;
    }

    /// Returns the current slice index.
    pub fn index(&mut self) -> usize {
        self.end
    }

    /// Returns the current position as a two value tuple of `(row, column)`.
    pub fn position(&mut self) -> (usize, usize) {
        (self.row, self.col)
    }

}

/// A type which represents the char iterator used by the lexer.
type Chars<'a> = std::str::CharIndices<'a>;