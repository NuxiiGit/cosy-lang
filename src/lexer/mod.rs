pub mod scanner;

use crate::diagnostics::error::{ Error, Session };
use crate::syntax::token::*;

use scanner::Scanner;

/// An iterator over a string slice which produces `Token`s.
pub struct Lexer<'a, 'b> {
    scanner : Scanner<'a>,
    sess : &'b Session,
    eof : bool
}
impl<'a, 'b> Lexer<'a, 'b> {
    /// Creates a new lexer from this string scanner and parser session.
    pub fn from(scanner : Scanner<'a>, sess : &'b Session) -> Self {
        Lexer {
            scanner,
            sess,
            eof : false
        }
    }
}