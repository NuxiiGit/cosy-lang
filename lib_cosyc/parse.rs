pub mod lex;
pub mod ident;

use lex::{ Lexer, TokenKind, LiteralKind, IdentifierKind };

use super::common::{
	Session,
	diagnostics::{ IssueTracker, error::Error, span::Span }
};

use std::fmt;

pub struct Parser<'a> {
	issues : &'a mut IssueTracker,
	lexer : Lexer<'a>
}
impl<'a> Parser<'a> {
	pub fn next_token(&mut self) -> TokenKind {
		let token = self.lexer.advance();
		let span = self.lexer.span();
		println!("{}: {:?}", span, token);
		self.issues.report(Error {
			reason : "super special error reason",
			span : span.clone()
		});
		token
	}
}
impl<'a> From<&'a mut Session> for Parser<'a> {
	fn from(sess : &'a mut Session) -> Self {
		let issues = &mut sess.issues;
		let lexer = Lexer::from(&sess.src);
		Self { issues, lexer }
	}
}

/// Represents information about the program.
#[derive(Debug)]
pub struct Program {
	pub body : Block
}

/// Represents a block of statements
#[derive(Debug)]
pub struct Block {
	pub stmts : Vec<Node<Stmt>>
}

/// Represents statement information.
#[derive(Debug, Clone)]
pub enum Stmt {
	Decl,
	Expr {
		expr : Node<Expr>
	}
}

/// Represents expression information.
#[derive(Debug, Clone)]
pub enum Expr {
	Variable,
	Value {
		kind : ValueKind
	},
	NoOp
}

/// Represents the different primitive variants.
#[derive(Debug, Clone)]
pub enum ValueKind {
	Integer
}

/// Represents a piece of data paired with a source position.
#[derive(Debug, Clone)]
pub struct Node<T : fmt::Debug + Clone> {
	pub content : T,
	pub span : Span
}