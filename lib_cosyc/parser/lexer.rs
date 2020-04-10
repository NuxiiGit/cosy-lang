use crate::scanner::{ Scanner, CharKind };
use crate::span::Span;

use std::mem;

/// A struct which converts a stream of characters into individual tokens.
pub struct Lexer<'a> {
	reader : Scanner<'a>,
	peeked : TokenKind,
	span : Span
}
impl<'a> Lexer<'a> {
	/// Creates a new lexer from this source code.
	pub fn from(src : &'a str) -> Self {
		let mut reader = Scanner::from(src);
		let peeked = reader.tokenise();
		let span = Span::new();
		Self { reader, peeked, span }
	}

	/// Returns a reference to the current peeked token.
	pub fn peek(&self) -> &TokenKind {
		&self.peeked
	}

	/// Returns ownership of the peeked token.
	pub fn next(&mut self) -> TokenKind {
		self.span.replicate(self.reader.span());
		let next = self.reader.tokenise();
		mem::replace(&mut self.peeked, next)
	}

	/// Returns the span of the previously returned `Result`.
	pub fn span(&self) -> &Span {
		&self.span
	}
}

impl<'a> Scanner<'a> {
	/// Returns the next token in the source.
	pub fn tokenise(&mut self) -> TokenKind {
		'search: loop {
			// skip preceeding whitespace
			self.clear_substr();
			let kind = match self.next() {
				// whitespace
				x if x.is_valid_whitespace() => {
					self.advance_while(CharKind::is_valid_whitespace);
					continue 'search;
				}
				// individual symbols
				CharKind::Equals if !self.peek().is_valid_operator() => TokenKind::Assign,
				CharKind::LeftParen => TokenKind::LeftParen,
				CharKind::RightParen => TokenKind::RightParen,
				CharKind::LeftBrace => TokenKind::LeftBrace,
				CharKind::RightBrace => TokenKind::RightBrace,
				CharKind::SemiColon => TokenKind::SemiColon,
				CharKind::Dollar => TokenKind::Dollar,
				CharKind::Backtick => TokenKind::Backtick,
				CharKind::Hashtag => TokenKind::Hashtag,
				CharKind::Address => TokenKind::Address,
				CharKind::EoF => TokenKind::EoF,
				// line comments
				CharKind::DoubleDash => {
					self.advance_until(CharKind::is_valid_ending);
					continue 'search;
				},
				// block comments
				CharKind::LeftDashedBrace => {
					let mut depth : u8 = 1;
					while depth >= 1 && depth < 255 {
						match self.next() {
							CharKind::LeftDashedBrace => depth += 1,
							CharKind::RightDashedBrace => depth -= 1,
							CharKind::EoF => break,
							_ => ()
						}
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
				// number literals
				x if x.is_valid_digit() => {
					self.advance_while(CharKind::is_valid_digit);
					TokenKind::Literal(LiteralKind::Integer)
				},
				// identifiers
				x if matches!(x, CharKind::Underscore) ||
						x.is_valid_graphic() ||
						x.is_valid_operator() => {
					let kind = match x {
						CharKind::Graphic |
								CharKind::Underscore => IdentifierKind::Alphabetic,
						CharKind::Bar => IdentifierKind::Bar,
						CharKind::Caret => IdentifierKind::Caret,
						CharKind::Ampersand => IdentifierKind::Ampersand,
						CharKind::Bang => IdentifierKind::Bang,
						CharKind::Equals => IdentifierKind::Equals,
						CharKind::LessThan => IdentifierKind::LessThan,
						CharKind::GreaterThan => IdentifierKind::GreaterThan,
						CharKind::Plus => IdentifierKind::Plus,
						CharKind::Minus => IdentifierKind::Minus,
						CharKind::Asterisk => IdentifierKind::Asterisk,
						CharKind::ForwardSlash => IdentifierKind::ForwardSlash,
						CharKind::Percent => IdentifierKind::Percent,
						_ => IdentifierKind::Other
					};
					let template = if matches!(x, CharKind::Underscore)
							{ self.peek() } else { &x };
					if template.is_valid_graphic() {
						self.advance_while(CharKind::is_valid_graphic);
					} else if template.is_valid_operator() {
						self.advance_while(CharKind::is_valid_operator);
					}
					loop {
						// all identifiers can end with any number of `'` (called "prime")
						self.advance_while(|x| matches!(x, CharKind::SingleQuote));
						// join alphanumeric identifiers and operators with underscores
						if matches!(self.peek(), CharKind::Underscore) {
							self.advance_while(|x| matches!(x, CharKind::Underscore));
							let peeked = self.peek();
							if peeked.is_valid_graphic() {
								self.advance_while(CharKind::is_valid_graphic);
							} else if peeked.is_valid_operator() {
								self.advance_while(CharKind::is_valid_operator);
							} else {
								break;
							}
						} else {
							break;
						}
					}
					// match substring for keywords
					match self.substr() {
						"var" => TokenKind::Var,
						"if" => TokenKind::If,
						"else" => TokenKind::Else,
						_ => TokenKind::Identifier(kind)
					}
				}
				// unknown symbol
				_ => TokenKind::Issue { reason : "unknown symbol" }
			};
			break kind;
		}
	}
}

/// An enum which describes available token types.
#[derive(PartialEq, Debug, Clone)]
pub enum TokenKind {
	Var,
	If,
	Else,
	Assign,
	LeftParen,
	RightParen,
	LeftBrace,
	RightBrace,
	SemiColon,
	Dollar,
	Backtick,
	Hashtag,
	Address,
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

	/// Returns `true` if the token can start an expression.
	pub fn is_nonterminal(&self) -> bool {
		self.is_alphabetic() || self.is_literal() || matches!(self,
				Self::LeftParen |
				Self::Hashtag)
	}
}

/// An enum which describes available identifier types.
#[derive(PartialEq, Debug, Clone)]
pub enum IdentifierKind {
	Alphabetic,
	Bar,
	Caret,
	Ampersand,
	Bang,
	Equals,
	LessThan,
	GreaterThan,
	Plus,
	Minus,
	Asterisk,
	ForwardSlash,
	Percent,
	Other
}

/// An enum which describes available literal types.
#[derive(PartialEq, Debug, Clone)]
pub enum LiteralKind {
	Integer
}