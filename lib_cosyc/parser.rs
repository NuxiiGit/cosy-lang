pub mod lexer;

use lexer::{ Lexer, TokenKind };

use crate::issues::{ Error, ErrorKind, IssueTracker };
use crate::span::Span;

/// Takes a lexer and uses it to construct a parse tree.
/// Syntax trees produced by this parser may or may not be invalid.
/// Therefore, you should check `issues` for any errors to verify whether the syntax tree is correct.
pub struct Parser<'a, 'e> {
	pub issues : &'e mut IssueTracker,
	pub lexer : Lexer<'a>
}
impl<'a, 'e> Parser<'a, 'e> {
	/// Creates a new parser from this issue tracker and this lexer.
	pub fn new(issues : &'e mut IssueTracker, lexer : Lexer<'a>) -> Self {
		Self { issues, lexer }
	}

	/// Parses tokens from a lexer, and then returns a program.
	pub fn parse_program(&mut self) -> Prog {
		let prog = Prog { stmts : Vec::new() };
		self.issues.report(Span::new().make_error(ErrorKind::Fatal, "test error"));
		prog
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
	Expr { expr : Expr },
	NoOp
}

/// A recursive enum which stores expression information.
#[derive(Debug, Clone)]
pub enum Expr {
	Integer { span : Span },
	Real { span : Span },
	Char { span : Span },
	Variable { span : Span }
}