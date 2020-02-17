use libcosyc_diagnostics::{ IssueTracker, Error, ErrorKind };
use libcosyc_syntax::token::*;

use std::iter::Peekable;

/// Takes a list of tokens and uses it to construct a parse tree.
pub struct Parser<'a, I : Iterator<Item=Token>> {
    tokens : Peekable<I>,
    issues : &'a IssueTracker
}
impl<'a, I : Iterator<Item=Token>> Parser<'a, I> {
    pub fn new(tokens : I, issues : &'a mut IssueTracker) -> Self {
        Self {
            tokens : tokens.peekable(),
            issues
        }
    }
}