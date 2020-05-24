pub mod lex;
pub mod ident;

use lex::{ Lexer, TokenKind, LiteralKind, IdentifierKind };
use ident::Identifier;

use super::common::{
	Session,
	diagnostics::{ IssueTracker, SourcePosition, SyntaxError, ErrorKind }
};

use std::{ mem, result };

/// Produces abstract syntax from concrete syntax. Reports any errors to the available `IssueTracker`.
pub struct Parser<'a> {
	issues : &'a mut IssueTracker,
	lexer : Lexer<'a>,
	peeked : TokenKind
}
impl<'a> Parser<'a> {
	/// Parses muliple declarations into a program.
	pub fn parse_program(&mut self) -> Program {
		let mut body = Vec::new();
		while !matches!(self.token(), TokenKind::EoF) {
			match self.parse_stmt() {
				Ok(stmt) => body.push(stmt),
				Err(err) => self.report(err)
			}
		}
		Program { body }
	}

	/// Parses an expression statement.
	pub fn parse_stmt(&mut self) -> Result<Stmt> {
		let mut requires_semicolon = false;
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
		let location = self.location();
		let kind = StmtKind::Expr { expr };
		Ok(Stmt { location, kind })
	}

	/// Parses any kind of expression.
	pub fn parse_expr(&mut self) -> Result<Expr> {
		self.parse_expr_terminal()
	}

	/// Parses literals, identifiers, and groupings of expressions.
	pub fn parse_expr_terminal(&mut self) -> Result<Expr> {
		let location = self.location();
		let kind = match self.matches(TokenKind::is_terminal) {
			Some(TokenKind::Identifier(ident, ..)) => {
				ExprKind::Variable { ident }
			},
			Some(TokenKind::Literal(kind)) => {
				let kind = match kind {
					LiteralKind::Integral(value) => ValueKind::Integer(value)
				};
				ExprKind::Value { kind }
			},
			Some(_) => return Err(SyntaxError::bug(location, "unknown terminal value")),
			_ => return self.parse_expr_groupings()
		};
		Ok(Expr { location, kind })
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
		if let Some(kind) = self.matches(p) {
			Ok(kind)
		} else {
			let location = self.location();
			self.advance();
			Err(SyntaxError::fatal(location, on_err))
		}
	}

	/// Advances the parser and returns `Some(TokenKind)` if some predicate is held,
	/// otherwise `None` is returned and the parser does not advance.
	pub fn matches(&mut self, p : fn(&TokenKind) -> bool) -> Option<TokenKind> {
		if p(self.token()) {
			Some(self.advance())
		} else {
			None
		}
	}

	/// Returns a reference to the current token kind.
	pub fn token(&self) -> &TokenKind {
		&self.peeked
	}

	/// Reports an error to the `IssueTracker`.
	pub fn report(&mut self, error : SyntaxError) {
		self.issues.report(error);
	}

	/// Returns the current location of the parser.
	pub fn location(&self) -> SourcePosition {
		self.lexer.cursor()
	}

	/// Advances the parser and returns the the previous lexeme.
	pub fn advance(&mut self) -> TokenKind {
		let next = self.lexer.advance();
		mem::replace(&mut self.peeked, next)
	}
}
impl<'a> From<&'a mut Session> for Parser<'a> {
	fn from(sess : &'a mut Session) -> Self {
		let issues = &mut sess.issues;
		let mut lexer = Lexer::from(&sess.src);
		let peeked = lexer.advance();
		Self { issues, lexer, peeked }
	}
}

/// Represents a parser result and failure case.
pub type Result<T> = result::Result<T, SyntaxError>;

/// Represents information about the program.
#[derive(Debug)]
pub struct Program {
	pub body : Vec<Stmt>
}

/// Represents a block of statements
#[derive(Debug)]
pub struct Block {
	pub stmts : Vec<Stmt>
}

/// Represents expression information.
#[derive(Debug)]
pub struct Stmt {
	pub location : SourcePosition,
	pub kind : StmtKind
}

/// Represents statement information.
#[derive(Debug)]
pub enum StmtKind {
	Expr {
		expr : Expr
	}
}

/// Represents expression information.
#[derive(Debug)]
pub struct Expr {
	pub location : SourcePosition,
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