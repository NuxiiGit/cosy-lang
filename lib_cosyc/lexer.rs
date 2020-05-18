pub mod scanner;

use scanner::{ CharReader, CharKind };

use crate::span::{ Node, Span };

use std::mem;

/// Converts a string into individual tokens.
pub struct Lexer<'a> {
	reader : CharReader<'a>,
	current : TokenKind
}
impl Lexer<'_> {
	/// Returns a reference to the current peeked token.
	pub fn token(&self) -> &TokenKind {
		&self.current
	}

	/// Returns ownership of the peeked token.
	pub fn advance(&mut self) -> Node<TokenKind> {
		let span = self.reader.span().clone();
		let next = self.reader.generate_token();
		let prev = mem::replace(&mut self.current, next);
		span.into_node(prev)
	}

	/// Returns the span of the peeked token.
	pub fn span(&self) -> &Span {
		self.reader.span()
	}
}
impl<'a> From<&'a str> for Lexer<'a> {
	fn from(src : &'a str) -> Self {
		let mut reader = CharReader::from(src);
		let current = reader.generate_token();
		Self { reader, current }
	}
}

impl CharReader<'_> {
	/// Returns the next token in the source.
	pub fn generate_token(&mut self) -> TokenKind {
	'search:
		loop {
			self.reset_span();
			let kind = match self.advance() {
				// whitespace
				x if x.is_valid_whitespace() => {
					self.advance_while(CharKind::is_valid_whitespace);
					continue 'search;
				}
				// individual symbols
				CharKind::LeftParen => TokenKind::LeftParen,
				CharKind::RightParen => TokenKind::RightParen,
				CharKind::LeftBrace => TokenKind::LeftBrace,
				CharKind::RightBrace => TokenKind::RightBrace,
				CharKind::SemiColon => TokenKind::SemiColon,
				CharKind::Backtick => TokenKind::Backtick,
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
						| CharKind::Graphic
						| CharKind::Underscore => IdentifierKind::Alphanumeric,
						| CharKind::Asterisk
						| CharKind::Solidus
						| CharKind::ReverseSolidus
						| CharKind::Percent => IdentifierKind::Multiplication,
						| CharKind::Plus
						| CharKind::Minus => IdentifierKind::Addition,
						| CharKind::GreaterThan
						| CharKind::LessThan => IdentifierKind::Comparison,
						| CharKind::Ampersand => IdentifierKind::And,
						| CharKind::Bar
						| CharKind::Caret => IdentifierKind::Or,
						| CharKind::Equals
						| CharKind::Bang 
						| CharKind::Hook
						| CharKind::Tilde => IdentifierKind::Equality,
						| CharKind::Dollar => IdentifierKind::Application,
						| _ => IdentifierKind::Other
					};
					if x.is_valid_graphic() {
						self.read_alphanumeric_identifier();
					} else if x.is_valid_operator() {
						self.read_operator_identifier();
					}
					// join alphanumeric identifiers and operators with underscores
					loop {
						if matches!(self.current(), CharKind::Underscore) {
							self.advance_while(|x| matches!(x, CharKind::Underscore));
							let peeked = self.current();
							if peeked.is_valid_graphic() {
								self.read_alphanumeric_identifier();
							} else if peeked.is_valid_operator() {
								self.read_operator_identifier();
							} else {
								break;
							}
						} else {
							break;
						}
					}
					// skip comment lexeme
					if self.holds_comment_lexeme() {
						self.advance_while(|x| !x.is_valid_newline());
						continue 'search;
					}
					// match substring for keywords
					match self.slice() {
						_ => TokenKind::Identifier(kind)
					}
				},
				// end of file
				CharKind::EoF => TokenKind::EoF,
				// unknown symbol
				_ => TokenKind::Unknown
			};
			break kind;
		}
	}

	fn read_alphanumeric_identifier(&mut self) {
		self.advance_while(CharKind::is_valid_graphic);
		// alphanumeric identifiers can end with any number of `'` (called "prime")
		self.advance_while(|x| matches!(x, CharKind::SingleQuote));
	}

	fn read_operator_identifier(&mut self) {
		self.advance_while(CharKind::is_valid_operator);
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
	Literal(LiteralKind),
	Identifier(IdentifierKind),
	EoF,
	Unknown
}
impl TokenKind {
	/// Returns `true` if the token is a literal value.
	pub fn is_literal(&self) -> bool {
		matches!(self, Self::Literal(..))
	}

	/// Returns `true` if the token is an identifier.
	pub fn is_identifier(&self) -> bool {
		matches!(self, Self::Identifier(..))
	}

	/// Returns `true` if the token is an alphabetic identifier.
	pub fn is_alphanumeric(&self) -> bool {
		matches!(self, Self::Identifier(IdentifierKind::Alphanumeric))
	}

	/// Returns `true` if the token is an operator identifier.
	pub fn is_operator(&self) -> bool {
		self.is_identifier() && !self.is_alphanumeric()
	}

	/// Returns `true` if the token is the end of the file.
	pub fn is_eof(&self) -> bool {
		matches!(self, Self::EoF)
	}
}

/// An enum which describes available literal types.
#[derive(PartialEq, Debug, Clone)]
pub enum LiteralKind {
	Integer
}

/// An enum which describes available identifier types.
#[derive(PartialEq, Debug, Clone)]
pub enum IdentifierKind {
	Alphanumeric,
	Multiplication,
	Addition,
	Comparison,
	And,
	Or,
	Equality,
	Application,
	Other
}