pub mod syntax;
pub mod diagnostics;

use std::fmt;
use std::rc::Rc;

/// A struct which stores information about some substring of a source file.
#[derive(Debug, Clone)]
pub struct Context {
    pub filepath : Rc<String>,
    pub src : String,
    pub line : usize
}
impl fmt::Display for Context {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "line {} in {}", self.line, self.filepath)
    }
}