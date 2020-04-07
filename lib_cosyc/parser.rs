pub mod lexer;

use lexer::{ Lexer, TokenKind };

use crate::issues::{ Error, ErrorKind, IssueTracker };
use crate::span::Span;

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

/// A struct which stores information about the parsed program.
#[derive(Debug)]
pub struct Prog {
	pub stmts : Block
}

/// A struct which represents a block of statements.
pub type Block = Vec<Stmt>;

/// A recursive enum which stores statement information.
#[derive(Debug, Clone)]
pub enum Stmt {
    Expr {
        expr : Expr
    },
    NoOp
}

/// A recursive enum which stores expression information.
#[derive(Debug, Clone)]
pub enum Expr {
    Value {
		kind : ValueKind,
		span : Span
    },
    Variable {
        span : Span
    },
}

/// Represents the different kinds of primitive values.
#[derive(PartialEq, Debug, Clone)]
pub enum ValueKind {
	Integer,
	Real,
	Char
}