pub mod lex;
pub mod ident;

use lex::{ Lexer, TokenKind, LiteralKind, IdentifierKind };
use ident::Identifier;

use super::common::{
    Session,
    diagnostics::{ IssueTracker, SourcePosition, Error, ErrorKind }
};

use std::{ mem, result };

/// Represents an error case.
pub type ParseResult<T> = Result<T, Error>;

/// Represents a kind of expression.
#[derive(Debug)]
pub enum ExprKind {
    Variable {
        ident : Identifier
    }
}

/// Represents expression information
#[derive(Debug)]
pub struct Expr {
    location : SourcePosition,
    kind : ExprKind
}

/// Produces a concrete syntax tree from concrete syntax.
pub struct Parser<'a> {
    lexer : Lexer<'a>,
    peeked : TokenKind,
    location : SourcePosition
}
impl<'a> Parser<'a> {
    /// Parses literals, identifiers, and groupings of expressions.
    pub fn parse_expr_terminal(&mut self) -> ParseResult<Expr> {
        let kind = match self.matches(TokenKind::is_terminal) {
            Some(TokenKind::Identifier(ident,
                    IdentifierKind::Alphanumeric)) => ExprKind::Variable { ident },
            _ => return Err(self.error(ErrorKind::Bug,
                    "unknown terminal value")),
        };
        let location = self.location();
        Ok(Expr { location, kind })
    }

    /// Advances the parser, but returns an error if some predicate isn't held.
	pub fn expects(&mut self, p : fn(&TokenKind) -> bool, on_err : &'static str) -> ParseResult<TokenKind> {
		if let Some(kind) = self.matches(p) {
			Ok(kind)
		} else {
			self.advance();
			Err(self.error(ErrorKind::Fatal, on_err))
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

	/// Attempts to unwrap a `ParseResult`. If an error occurs, it is reported to the parser and
	/// panic recovery is applied to skip offending tokens. `None` is returned in the case
	/// where the result variant was `Err`.
	pub fn synchronise<T>(&mut self, parse_result : ParseResult<T>, issues : &mut IssueTracker) -> Option<T> {
		match parse_result {
			Ok(x) => Some(x),
			Err(err) => {
				issues.report(err);
				loop {
					if self.matches(|x| matches!(x, TokenKind::SemiColon)).is_some() {
						break;
					} else if matches!(self.token(),
							TokenKind::Let
							| TokenKind::EoF) {
						break;
					}
					self.advance();
				}
				None
			}
		}
	}

	/// Creates an error at the current parser location.
	pub fn error(&self, kind : ErrorKind, reason : &'static str) -> Error {
		let location = self.location();
		Error { kind, reason, location }
	}

	/// Returns a reference to the current token kind.
	pub fn token(&self) -> &TokenKind {
		&self.peeked
	}

	/// Returns the current location of the parser.
	pub fn location(&self) -> SourcePosition {
		self.location
	}

	/// Advances the parser and returns the the previous lexeme.
	pub fn advance(&mut self) -> TokenKind {
		self.location = self.lexer.cursor();
		let next = self.lexer.advance();
		mem::replace(&mut self.peeked, next)
	}
}
impl<'a> From<Lexer<'a>> for Parser<'a> {
    fn from(mut lexer : Lexer<'a>) -> Self {
		let peeked = lexer.advance();
		let location = 0;
		Self { lexer, peeked, location }
    }
}

/*
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
	peeked : TokenKind,
	location : SourcePosition
}
impl<'a> Parser<'a> {
	/// Parses muliple declarations into a program.
	pub fn parse_program(&mut self) -> Program {
		let mut body = Vec::new();
		while !matches!(self.token(), TokenKind::EoF) {
			let result = self.parse_stmt();
			if let Some(stmt) = self.synchronise(result) {
				body.push(stmt)
			}
		}
		Program { body }
	}

	/// Parses a statement.
	pub fn parse_stmt(&mut self) -> Result<Stmt> {
		if matches!(self.token(), TokenKind::Let) {
			// declaration statement
			let decl = self.parse_decl()?;
			Ok(Stmt::Decl { decl })
		} else {
			// expression statement
			let expr = self.parse_expr()?;
			self.expects(|x| matches!(x, TokenKind::SemiColon), "expected semicolon after statement")?;
			Ok(Stmt::Expr { expr })
		}
	}

	/// Parses a declaration.
	pub fn parse_decl(&mut self) -> Result<Decl> {
		self.expects(|x| matches!(x, TokenKind::Let), "expected `let` in declaration")?;
		let ident = match self.advance() {
			TokenKind::Identifier(ident, ..) => ident,
			_ => return Err(self.error(ErrorKind::Bug, "incompatible declaration value")),
		};
		self.expects(|x| matches!(x, TokenKind::Assign), "expected assignment operator `=` in declaration")?;
		let value = self.parse_expr()?;
		self.expects(|x| matches!(x, TokenKind::SemiColon), "expected semi-colon after declaration")?;
		Ok(Decl::Variable { ident, value })
	}

	/// Parses any kind of expression.
	pub fn parse_expr(&mut self) -> Result<Expr> {
		self.parse_expr_terminal()
	}

	/// Parses literals, identifiers, and groupings of expressions.
	pub fn parse_expr_terminal(&mut self) -> Result<Expr> {
		let kind = match self.matches(TokenKind::is_terminal) {
			Some(TokenKind::Identifier(ident, ..)) => {
				ValueKind::Variable { ident }
			},
			Some(TokenKind::Literal(kind)) => {
				let kind = match kind {
					LiteralKind::Integral(value) => ConstantKind::Integral(value)
				};
				ValueKind::Constant { kind }
			},
			Some(_) => return Err(self.error(ErrorKind::Bug, "unknown terminal value")),
			_ => return self.parse_expr_groupings()
		};
		let location = self.location();
		let kind = ExprKind::Value { location, kind };
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
		if let Some(kind) = self.matches(p) {
			Ok(kind)
		} else {
			self.advance();
			Err(self.error(ErrorKind::Fatal, on_err))
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

	/// Attempts to unwrap a result. If an error occurs, it is reported to the parser and
	/// panic recovery is applied to skip offending tokens. `None` is returned in the case
	/// where the result variant was `Err`.
	pub fn synchronise<T>(&mut self, parse_result : Result<T>) -> Option<T> {
		match parse_result {
			Ok(x) => Some(x),
			Err(err) => {
				self.issues.report(err);
				loop {
					if self.matches(|x| matches!(x, TokenKind::SemiColon)).is_some() {
						break;
					} else if matches!(self.token(),
							TokenKind::Let
							| TokenKind::EoF) {
						break;
					}
					self.advance();
				}
				None
			}
		}
	}

	/// Creates an error at the current parser location.
	pub fn error(&self, kind : ErrorKind, reason : &'static str) -> SyntaxError {
		let location = self.location();
		SyntaxError { kind, reason, location }
	}

	/// Returns a reference to the current token kind.
	pub fn token(&self) -> &TokenKind {
		&self.peeked
	}

	/// Returns the current location of the parser.
	pub fn location(&self) -> SourcePosition {
		self.location
	}

	/// Advances the parser and returns the the previous lexeme.
	pub fn advance(&mut self) -> TokenKind {
		self.location = self.lexer.cursor();
		let next = self.lexer.advance();
		mem::replace(&mut self.peeked, next)
	}
}
impl<'a> From<&'a mut Session> for Parser<'a> {
	fn from(sess : &'a mut Session) -> Self {
		let issues = &mut sess.issues;
		let mut lexer = Lexer::from(&sess.src);
		let peeked = lexer.advance();
		let location = 0;
		Self { issues, lexer, peeked, location }
	}
}

/// Represents a parser result and failure case.
pub type Result<T> = result::Result<T, SyntaxError>;

/// Represents information about the program.
#[derive(Debug)]
pub struct Program {
	pub body : Vec<Stmt>
}

/// Represents statement information.
#[derive(Debug)]
pub enum Stmt {
	Decl {
		decl : Decl
	},
	Expr {
		expr : Expr
	}
}

/// Represents declaration information.
#[derive(Debug)]
pub enum Decl {
	Variable {
		ident : Identifier,
		value : Expr
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
	Value {
		location : SourcePosition,
		kind : ValueKind
	}
}

/// Represents the different kinds of terminal expression.
#[derive(Debug)]
pub enum ValueKind {
	Variable {
		ident : Identifier
	},
	Constant {
		kind : ConstantKind
	}
}

/// Represents the different primitive variants.
#[derive(Debug)]
pub enum ConstantKind {
	Integral(usize)
}*/
