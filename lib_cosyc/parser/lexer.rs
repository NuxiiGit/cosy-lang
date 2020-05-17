use crate::scanner::{ CharReader, CharKind };
use crate::span::Span;

use std::mem;

/// Converts a string into individual tokens.
pub struct Lexer<'a> {
	reader : CharReader<'a>,
	peeked : TokenKind
}
impl Lexer<'_> {
	/// Returns a reference to the current peeked token.
	pub fn token(&self) -> &TokenKind {
		&self.peeked
	}

	/// Returns ownership of the peeked token.
	pub fn advance(&mut self) -> TokenKind {
		let next = self.reader.generate_token();
		mem::replace(&mut self.peeked, next)
	}

	/// Returns the span of the peeked token.
	pub fn span(&self) -> &Span {
		self.reader.span()
	}
}
impl<'a> From<&'a str> for Lexer<'a> {
	fn from(src : &'a str) -> Self {
		let mut reader = CharReader::from(src);
		let peeked = reader.generate_token();
		Self { reader, peeked }
	}
}

impl CharReader<'_> {
	/// Returns the next token in the source.
	pub fn generate_token(&mut self) -> TokenKind {
	'search:
		loop {
			self.clear_substr();
			let kind = match self.next() {
				// whitespace
				x if x.is_valid_whitespace() => {
					self.advance_while(CharKind::is_valid_whitespace);
					continue 'search;
				}
				// line comments
				CharKind::Minus if matches!(self.peek(), CharKind::Minus) => {
					self.advance_while(|x| !CharKind::is_valid_newline(x));
					continue 'search;
				},
				// block comments
				CharKind::LeftBrace if matches!(self.peek(), CharKind::Minus) => {
					let mut depth : u8 = 1;
					while depth >= 1 && depth < 255 {
						match self.next() {
							CharKind::LeftBrace if matches!(self.peek(), CharKind::Minus) => depth += 1,
							CharKind::Minus if matches!(self.peek(), CharKind::RightBrace) => depth -= 1,
							CharKind::EoF => break,
							_ => continue
						}
						self.next();
					}
					let reason = if depth >= 1 {
						"unterminated block comment"
					} else if depth == 255 {
						"nested block comment exceeds depth limit"
					} else {
						continue 'search
					};
					TokenKind::Issue { reason }
				},
				// individual symbols
				CharKind::LeftParen => TokenKind::LeftParen,
				CharKind::RightParen => TokenKind::RightParen,
				CharKind::LeftBrace => TokenKind::LeftBrace,
				CharKind::RightBrace => TokenKind::RightBrace,
				CharKind::SemiColon => TokenKind::SemiColon,
				CharKind::Backtick => TokenKind::Backtick,
				CharKind::EoF => TokenKind::EoF,
				// number literals
				x if x.is_valid_digit() => {
					self.advance_while(CharKind::is_valid_digit);
					TokenKind::Literal(LiteralKind::Integer)
				},
				// unknown symbol
				_ => TokenKind::Issue { reason : "unknown symbol" }
			};
			break kind;
		}
	}
}

/// Represents available token types.
#[derive(PartialEq, Debug, Clone)]
pub enum TokenKind {
	LeftParen,
	RightParen,
	LeftBrace,
	RightBrace,
	SemiColon,
	Backtick,
	Identifier(IdentifierKind),
	Literal(LiteralKind),
	EoF,
	Issue { reason : &'static str }
}
impl TokenKind {
	/// Returns `true` if the token is an identifier.
	pub fn is_identifier(&self) -> bool {
		matches!(self, Self::Identifier(..))
	}

	/// Returns `true` if the token is an alphabetic identifier.
	pub fn is_alphabetic(&self) -> bool {
		matches!(self, Self::Identifier(IdentifierKind::Alphabetic))
	}

	/// Returns `true` if the token is an operator identifier.
	pub fn is_operator(&self) -> bool {
		self.is_identifier() && !self.is_alphabetic()
	}

	/// Returns `true` if the token is a literal value.
	pub fn is_literal(&self) -> bool {
		matches!(self, Self::Literal(..))
	}

	/// Returns `true` if the token is the end of the file.
	pub fn is_eof(&self) -> bool {
		matches!(self, Self::EoF)
	}
}

/// An enum which describes available identifier types.
#[derive(PartialEq, Debug, Clone)]
pub enum IdentifierKind {
	Alphabetic,
	Multiplication,
	Addition,
	Comparison,
	And,
	Or,
	Equality,
	Other,
	Application
}

/// An enum which describes available literal types.
#[derive(PartialEq, Debug, Clone)]
pub enum LiteralKind {
	Integer
}