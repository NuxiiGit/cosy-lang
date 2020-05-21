pub mod lex;
pub mod ident;

use lex::{ Lexer, TokenKind, LiteralKind, IdentifierKind };
use ident::Identifier;

use super::common::{
	Session,
	diagnostics::{
		IssueTracker,
		error::{ Error, ErrorKind },
		span::{ Span, Cursor }
	}
};

use std::{ fmt, mem, result };

/// Produces abstract syntax from concrete syntax. Reports any errors to the available `IssueTracker`.
pub struct Parser<'a> {
	issues : &'a mut IssueTracker,
	lexer : Lexer<'a>,
	current : TokenKind
}
impl<'a> Parser<'a> {
	/// Parses any kind of statement.
	pub fn parse_stmt(&mut self) -> Result<Stmt> {
		let mut requires_semicolon = false;
		let begin = self.cursor();
		let content = match self.token() {
			_ => {
				// expression statements require semicolons
				requires_semicolon = true;
				self.parse_expr()
			},
		}?;
		if requires_semicolon {
			self.expects(|x| matches!(x, TokenKind::SemiColon), "expected semicolon after statement")?;
		}
		let end = self.cursor();
		let span = Span { begin, end };
		let expr = Node { content, span };
		Ok(Stmt::Expr { expr })
	}

	/// Parses any kind of expression.
	pub fn parse_expr(&mut self) -> Result<Expr> {
		self.parse_expr_terminal()
	}

	/// Parses literals, identifiers, and groupings of expressions.
	pub fn parse_expr_terminal(&mut self) -> Result<Expr> {
		match self.token() {
			TokenKind::Identifier(ident, ..) => {
				let ident = *ident;
				self.advance();
				Ok(Expr::Variable { ident })
			},
			TokenKind::Literal(kind) => {
				let kind = match kind {
					LiteralKind::Integral(value) => ValueKind::Integer(*value)
				};
				self.advance();
				Ok(Expr::Value { kind })
			},
			_ => self.parse_expr_groupings()
		}
	}

	/// Parses groupings of expressions.
	pub fn parse_expr_groupings(&mut self) -> Result<Expr> {
		self.expects(|x| matches!(x, TokenKind::LeftParen), "malformed expression")?;
		let node = self.parse_expr()?;
		self.expects(|x| matches!(x, TokenKind::RightParen), "expected closing parenthesis in grouping")?;
		Ok(node)
	}

	/// Advances the parser, but returns an error if some predicate isn't held.
	pub fn expects(&mut self, p : fn(&TokenKind) -> bool, on_err : &'static str) -> Result<TokenKind> {
		if p(self.token()) {
			Ok(self.advance())
		} else {
			let error = Error {
				reason : on_err,
				span : self.span(),
				kind : ErrorKind::Fatal
			};
			self.advance();
			Err(error)
		}
	}

	/// Returns a reference to the current token kind.
	pub fn token(&self) -> &TokenKind {
		&self.current
	}

	/// Returns the current token span.
	pub fn span(&self) -> Span {
		self.lexer.span().clone()
	}

	/// Returns the left-most cursor of the lexer.
	pub fn cursor(&self) -> Cursor {
		let cursor = &self.lexer.span().begin;
		cursor.clone()
	}

	/// Advances the parser and returns the the previous lexeme.
	pub fn advance(&mut self) -> TokenKind {
		let next = self.lexer.advance();
		mem::replace(&mut self.current, next)
	}

	/// Inserts a warning into to the `IssueTracker`.
	pub fn warn(&mut self, reason : &'static str) {
		self.issues.report(Error {
			reason,
			span : self.lexer.span().clone(),
			kind : ErrorKind::Warning
		});
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
pub type Result<T> = result::Result<T, Error>;

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
	Declr {
		ident : Identifier
	},
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

struct NodeBuilder<'a, 'p> {
	parser : &'a Parser<'p>,
	cursor : Cursor
}
impl<'a, 'p> NodeBuilder<'a, 'p> {
	fn start(parser : &'a Parser<'p>) -> Self {
		let cursor = parser.lexer.span().begin.clone();
		NodeBuilder { parser, cursor }
	}

	fn end<T>(self, content : T) -> Node<T> {
		let begin = self.cursor;
		let end = self.parser.lexer.span().begin.clone();
		let span = Span { begin, end };
		Node { content, span }
	}
}

/// Represents a piece of data paired with a source position.
pub struct Node<T> {
	pub content : T,
	pub span : Span
}
impl<T : fmt::Debug> fmt::Debug for Node<T> {
	fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
		write!(out, "{{span={}}} {:?}", self.span, self.content)
    }
}