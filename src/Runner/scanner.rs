#![allow(dead_code)]

/// A struct which allows you to iterate and slice `&str` types.
pub struct Scanner<'a> {
    context : &'a str,
    chars : Chars<'a>,
    current : Option<char>,
    index : usize,
    index_next : usize,
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
            index : 0,
            index_next : 0,
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
        self.index = self.index_next;
        self.index_next = if let Some((i, x)) = self.chars.next() {
            self.current = Some(x);
            i
        } else {
            self.current = None;
            self.context.len()
        };
        current
    }

    /// Returns a slice of the scanner context.
    pub fn slice(&self, left : usize, right : usize) -> &'a str {
        &self.context[left..right]
    }

    /// Returns the current left-most index.
    pub fn index_left(&mut self) -> usize {
        self.index
    }
    
    /// Returns the current right-most index.
    pub fn index_right(&mut self) -> usize {
        self.index_next
    }

    /// Returns the current row of the scanner.
    pub fn row(&mut self) -> usize {
        self.row
    }

    /// Returns the current column of the scanner.
    pub fn column(&mut self) -> usize {
        self.col
    }

}

/// A type which represents the char iterator used by the lexer.
type Chars<'a> = std::str::CharIndices<'a>;