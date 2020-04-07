pub mod lexer;

use lexer::{ Lexer, TokenKind };

use crate::issues::{ Error, ErrorKind, IssueTracker };

/// Takes a lexer and uses it to construct a parse tree.
pub struct Parser<'e> {
    issues : &'e mut IssueTracker
}
impl<'a, 'e> Parser<'e> {
	/// Creates a new parser from this issue tracker.
	pub fn new(issues : &'e mut IssueTracker) -> Self {
		Self { issues }
	}

	/// Parses tokens from a lexer, and then returns a program.
	pub fn parse(&self, lexer : &Lexer<'a>) -> Prog {
		unimplemented!()
	}
}

#[derive(Debug)]
pub struct Prog;