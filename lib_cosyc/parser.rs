pub mod lexer;

use lexer::{ Lexer, TokenKind, LiteralKind, IdentifierKind };

use crate::issues::{ ErrorKind, IssueTracker };
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
		let prog = Prog { stmts : Vec::new() };
		self.issues.report(Span::new().make_error(ErrorKind::Fatal, "test error"));
		Some(prog)
	}

	/// Parses expression literals and identifiers.
	pub fn parse_expr_terminal(&mut self) -> Option<Expr> {
		if let Some(literal) = self.advance_if(TokenKind::is_literal) {
			let span = self.lexer.span().clone();
			Some(match literal {
				TokenKind::Literal(LiteralKind::Character) => Expr::Char { span },
				TokenKind::Literal(LiteralKind::Integer) => Expr::Integer { span },
				TokenKind::Literal(LiteralKind::Real) => Expr::Real { span },
				_ => unreachable!()
			})
		} else if self.advance_if(TokenKind::is_identifier).is_some() {
			let span = self.lexer.span().clone();
			Some(Expr::Variable { span })
		} else {
			self.lexer.span().make_error(ErrorKind::Fatal, "malformed expression");
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
			let span = self.lexer.span();
			self.issues.report(span.make_error(ErrorKind::Fatal, on_err));
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
				let span = self.lexer.span();
				self.issues.report(span.make_error(ErrorKind::Fatal, reason));
				None
			},
			token => Some(token)
		}
	}

	/// Returns `true` if the next token satisfies some predicate.
	fn satisfies(&self, p : fn(&TokenKind) -> bool) -> bool {
		p(self.lexer.peek())
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