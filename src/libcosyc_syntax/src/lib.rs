pub mod token;

use std::fmt;

/// A struct which stores information about some substring of a source file.
#[derive(Debug, Clone)]
pub struct Context<'a> {
    pub src : &'a str,
    pub line : usize
}
impl fmt::Display for Context<'_> {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "line {}", self.line)
    }
}