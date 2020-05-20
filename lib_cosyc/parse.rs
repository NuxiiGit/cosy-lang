pub mod lex;
pub mod ident;

use lex::{ Lexer, TokenKind, LiteralKind, IdentifierKind };
use ident::Identifier;

use super::common::{
	Session,
	diagnostics::{ IssueTracker, error::Error, span::Span }
};

use std::{ mem, result };

/// Produces abstract syntax from concrete syntax. Reports any errors to the available `IssueTracker`.
pub struct Parser<'a> {
	issues : &'a mut IssueTracker,
	lexer : Lexer<'a>,
	current : TokenKind
}
impl<'a> Parser<'a> {
	/// Parses any kind of expression.
	pub fn parse_expr(&mut self) -> Result<Expr> {
		self.parse_expr_terminal()
	}

	/// Parses literals, identifiers, and groupings of expressions.
	pub fn parse_expr_terminal(&mut self) -> Result<Expr> {
		match self.token() {
			TokenKind::Identifier(ident, ..) => {
				let ident = *ident;
				let node = self.advance();
				Ok(node.into(Expr::Variable { ident }))
			},
			TokenKind::Literal(kind) => {
				let kind = match kind {
					LiteralKind::Integral(value) => ValueKind::Integer(*value)
				};
				let node = self.advance();
				Ok(node.into(Expr::Value { kind }))
			},
			_ => self.parse_expr_groupings()
		}
	}

	/// Parses groupings of expressions.
	pub fn parse_expr_groupings(&mut self) -> Result<Expr> {
		self.expects(TokenKind::LeftParen, "malformed expression")?;
		let expr = self.parse_expr()?;
		self.expects(TokenKind::RightParen, "expected closing parenthesis in grouping")?;
		Ok(expr)
	}

	/// Advances the parser, but returns an error if some predicate isn't held.
	pub fn expects(&mut self, kind : TokenKind, on_err : &'static str) -> Result<TokenKind> {
		let node = self.advance();
		if node.content == kind {
			Ok(node)
		} else {
			Err(Error {
				reason : on_err,
				span : node.span
			})
		}
	}

	/// Returns a reference to the current token kind.
	pub fn token(&self) -> &TokenKind {
		&self.current
	}

	/// Advances the parser and returns the `Node` of the previous lexeme.
	pub fn advance(&mut self) -> Node<TokenKind> {
		let span = self.lexer.span().clone();
		let next = self.lexer.advance();
		let prev = mem::replace(&mut self.current, next);
		Node {
			content : prev,
			span
		}
	}
}
impl<'a> From<&'a mut Session> for Parser<'a> {
	fn from(sess : &'a mut Session) -> Self {
		let issues = &mut sess.issues;
		let mut lexer = Lexer::from(&sess.src);
		let current = lexer.advance();
		Self { issues, lexer, current }
	}
}

/// Represents a parser result and failure case.
pub type Result<T> = result::Result<Node<T>, Error>;

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
#[derive(Debug)]
pub enum Stmt {
	Decl,
	Expr {
		expr : Node<Expr>
	}
}

/// Represents expression information.
#[derive(Debug)]
pub enum Expr {
	Variable {
		ident : Identifier
	},
	Value {
		kind : ValueKind
	},
	NoOp
}

/// Represents the different primitive variants.
#[derive(Debug)]
pub enum ValueKind {
	Integer(usize)
}

/// Represents a piece of data paired with a source position.
#[derive(Debug)]
pub struct Node<T> {
	pub content : T,
	pub span : Span
}
impl<T> Node<T> {
	/// Converts a Node of type `T` into to a node of type `S`.
	/// The content of the previous span is consumed and discarded.
	pub fn into<S>(self, content : S) -> Node<S> {
		let span = self.span;
		Node { content, span }
	}
}