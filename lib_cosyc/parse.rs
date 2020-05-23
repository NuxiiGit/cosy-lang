pub mod lex;
pub mod ident;

use lex::{ Lexer, TokenKind, LiteralKind, IdentifierKind };
use ident::Identifier;

use super::common::{
	Session,
	diagnostics::{
		IssueTracker, SourcePosition,
		error::{ Error, ErrorKind }
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
	/// Parses an expression statement.
	pub fn parse_stmt(&mut self) -> Result<Stmt> {
		let mut requires_semicolon = false;
		let position = self.lexer.cursor();
		let expr = match self.token() {
			_ => {
				// expression statements always require semicolons
				requires_semicolon = true;
				self.parse_expr()
			},
		}?;
		if requires_semicolon {
			self.expects(|x| matches!(x, TokenKind::SemiColon), "expected semicolon after statement")?;
		}
		Ok(Stmt {
			kind : StmtKind::Expr { expr }
		})
	}

	/// Parses any kind of expression.
	pub fn parse_expr(&mut self) -> Result<Expr> {
		self.parse_expr_terminal()
	}

	/// Parses literals, identifiers, and groupings of expressions.
	pub fn parse_expr_terminal(&mut self) -> Result<Expr> {
		let position = self.lexer.cursor();
		let kind = match self.token() {
			TokenKind::Identifier(ident, ..) => {
				let ident = *ident;
				ExprKind::Variable { ident }
			},
			TokenKind::Literal(kind) => {
				let kind = match kind {
					LiteralKind::Integral(value) => ValueKind::Integer(*value)
				};
				ExprKind::Value { kind }
			},
			_ => return self.parse_expr_groupings()
		};
		self.advance();
		Ok(Expr { kind })
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
				kind : ErrorKind::Fatal
			};
			Err(error)
		}
	}

	/// Returns a reference to the current token kind.
	pub fn token(&self) -> &TokenKind {
		&self.current
	}

	/// Advances the parser and returns the the previous lexeme.
	pub fn advance(&mut self) -> TokenKind {
		let next = self.lexer.advance();
		mem::replace(&mut self.current, next)
	}

	/// Inserts a warning into to the `IssueTracker`.
	pub fn warn(&mut self, reason : &'static str) {
		let kind = ErrorKind::Warning;
		self.report(Error { reason, kind });
	}

	/// Reports an error to the `IssueTracker`.
	pub fn report(&mut self, error : Error) {
		self.issues.report(self.cursor(), error);
	}

	/// Returns the current cursor of the parser.
	pub fn cursor(&self) -> SourcePosition {
		self.lexer.cursor()
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
	pub stmts : Vec<Stmt>
}

/// Represents expression information.
#[derive(Debug)]
pub struct Stmt {
	pub kind : StmtKind
}

/// Represents statement information.
#[derive(Debug)]
pub enum StmtKind {
	Declr {
		ident : Identifier
	},
	Expr {
		expr : Expr
	}
}

/// Represents expression information.
#[derive(Debug)]
pub struct Expr {
	pub kind : ExprKind
}

/// Represents a kind of expression.
#[derive(Debug)]
pub enum ExprKind {
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