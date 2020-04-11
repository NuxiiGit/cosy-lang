pub mod lexer;

use lexer::{ Lexer, TokenKind, LiteralKind, IdentifierKind };

use crate::issues::{ Error, IssueTracker };
use crate::span::Span;

/// Takes a lexer and uses it to construct a parse tree.
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
	pub fn parse_program(&mut self) -> Option<Prog> {
		let mut stmts = Vec::new();
		let mut errors_occured = false;
		while !self.satisfies(|x| matches!(x, TokenKind::EoF)) {
			if let Some(stmt) = self.parse_stmt() {
				stmts.push(stmt);
			} else {
				errors_occured = true;
				self.synchronise();
			}
		}
		if errors_occured
				{ None } else { Some(Prog { stmts }) }
	}

	/// Parses a single statement.
	pub fn parse_stmt(&mut self) -> Option<Stmt> {
		let expr = self.parse_expr()?;
		self.expects(|x| matches!(x, TokenKind::SemiColon), "expected semi-colon after expression statement");
		Some(Stmt::Expr { expr })
	}

	/// Parses a single expression.
	pub fn parse_expr(&mut self) -> Option<Expr> {
		self.parse_expr_terminal()
	}

	/// Parses expression literals and identifiers.
	pub fn parse_expr_terminal(&mut self) -> Option<Expr> {
		if let Some(literal) = self.advance_if(TokenKind::is_literal) {
			let span = self.lexer.span().clone();
			Some(match literal {
				TokenKind::Literal(LiteralKind::Integer) => Expr::Integer { span },
				_ => unreachable!()
			})
		} else if self.advance_if(TokenKind::is_identifier).is_some() {
			let span = self.lexer.span().clone();
			Some(Expr::Variable { span })
		} else {
			self.report("malformed expression");
			None
		}
	}

	/// Advances the parser until a stable token is found.
	fn synchronise(&mut self) {
		loop {
			if self.advance_if(|x| matches!(x, TokenKind::SemiColon)).is_some() {
				break;
			} else if self.satisfies(|x| matches!(x,
					TokenKind::Var |
					TokenKind::EoF)) {
				break;
			}
			self.advance();
		}
	}

	/// Advances the parser, but reports an error if some predicate isn't held.
	fn expects(&mut self, p : fn(&TokenKind) -> bool, on_err : &'static str) -> Option<TokenKind> {
		if self.satisfies(p) {
			self.advance()
		} else {
			self.advance()?;
			self.report(on_err);
			None
		}
	}

	/// Advances the parser only if the next token satisfies some predicate.
	fn advance_if(&mut self, p : fn(&TokenKind) -> bool) -> Option<TokenKind> {
		if self.satisfies(p) {
			self.advance()
		} else {
			None
		}
	}

	/// Advances the parser, but registers any issue tokens as fatal errors.
	/// `Option` is used to unwind.
	fn advance(&mut self) -> Option<TokenKind> {
		match self.lexer.next() {
			TokenKind::Issue { reason } => {
				self.report(reason);
				None
			},
			token => Some(token)
		}
	}

	/// Returns `true` if the next token satisfies some predicate.
	fn satisfies(&self, p : fn(&TokenKind) -> bool) -> bool {
		p(self.lexer.peek())
	}

	/// Reports an error.
	fn report(&mut self, reason : &'static str) {
		self.issues.report(Error {
			reason,
			span : self.lexer.span().clone()
		})
	}
}

/// A struct which provides a way of checking whether a value satisfies one or more conditions.
pub struct ParserComparator<'p, 'a, 'e> {
	parser : &'p mut Parser<'a, 'e>,
	satisfied : bool
}
impl<'p, 'a, 'e> ParserComparator<'p, 'a, 'e> {
	/// Creates a new comparator from this value.
	pub fn from(parser : &'p mut Parser<'a, 'e>) -> Self {
		Self {
			parser,
			satisfied : false
		}
	}

	/// Satisfies the value if it is equal to the target value.
	pub fn equals(mut self, token : TokenKind) -> Self {
		if !self.satisfied {
			self.satisfied = *self.parser.lexer.peek() == token;
		}
		self
	}

	/// Satisfies the value if it holds for some predicate.
	pub fn satisfies(mut self, p : fn(&TokenKind) -> bool) -> Self {
		if !self.satisfied {
			self.satisfied = p(&self.parser.lexer.peek());
		}
		self
	}

	/// Negates the condition.
	pub fn not(mut self) -> Self {
		self.satisfied = !self.satisfied;
		self
	}

	/// Reports an error if the condition is not satisfied.
	pub fn expects(self, reason : &'static str) -> Self {
		if !self.satisfied {
			self.parser.report(reason);
		}
		self
	}
	
	/// Consumes the comparator and returns the token as an optional type.
	pub fn check(self) -> Option<TokenKind> {
		if self.satisfied {
			self.parser.advance()
		} else {
			None
		}
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
	Variable { span : Span }
}