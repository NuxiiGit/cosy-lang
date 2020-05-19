use crate::scanner::{ CharReader, CharKind };
use crate::span::Span;

use super::ident::{ NameTable, Identifier };

/// Converts a string into individual tokens.
pub struct Lexer<'a> {
	reader : CharReader<'a>,
	name_table : NameTable<'a>
}
impl Lexer<'_> {
	/// Returns the span of the current token.
	pub fn span(&self) -> &Span {
		self.reader.span()
	}

	/// Returns the next token in the source.
	pub fn advance(&mut self) -> TokenKind {
	'search:
		loop {
			self.reader.reset_span();
			let kind = match self.reader.advance() {
				// whitespace
				x if x.is_valid_whitespace() => {
					self.reader.advance_while(CharKind::is_valid_whitespace);
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
					self.reader.advance_while(CharKind::is_valid_digit);
					let digit = self.reader.slice().parse::<usize>().unwrap();
					TokenKind::Literal(LiteralKind::Integral(digit))
				},
				// identifiers
				x if matches!(x, CharKind::Underscore) ||
						x.is_valid_graphic() ||
						x.is_valid_operator() => {
					let kind = match x {
							CharKind::Graphic
						| CharKind::Underscore => IdentifierKind::Alphanumeric,
							CharKind::Asterisk
						| CharKind::Solidus
						| CharKind::ReverseSolidus
						| CharKind::Percent => IdentifierKind::Multiplication,
							CharKind::Plus
						| CharKind::Minus => IdentifierKind::Addition,
							CharKind::GreaterThan
						| CharKind::LessThan => IdentifierKind::Comparison,
							CharKind::Ampersand => IdentifierKind::And,
							CharKind::Bar
						| CharKind::Caret => IdentifierKind::Or,
							CharKind::Equals
						| CharKind::Bang 
						| CharKind::Hook
						| CharKind::Tilde => IdentifierKind::Equality,
							CharKind::Dollar => IdentifierKind::Application,
							_ => IdentifierKind::Other
					};
					if x.is_valid_graphic() {
						self.read_alphanumeric_identifier();
					} else if x.is_valid_operator() {
						self.read_operator_identifier();
					}
					// join alphanumeric identifiers and operators with underscores
					loop {
						if matches!(self.reader.current(), CharKind::Underscore) {
							self.reader.advance_while(|x| matches!(x, CharKind::Underscore));
							let peeked = self.reader.current();
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
					if self.reader.holds_comment_lexeme() {
						self.reader.advance_while(|x| !x.is_valid_newline());
						continue 'search;
					}
					// match substring for keywords
					match self.reader.slice() {
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
		self.reader.advance_while(CharKind::is_valid_graphic);
		// alphanumeric identifiers can end with any number of `'` (called "prime")
		self.reader.advance_while(|x| matches!(x, CharKind::SingleQuote));
	}

	fn read_operator_identifier(&mut self) {
		self.reader.advance_while(CharKind::is_valid_operator);
	}
}
impl<'a> From<&'a str> for Lexer<'a> {
	fn from(src : &'a str) -> Self {
		let mut reader = CharReader::from(src);
		let name_table = NameTable::new();
		Self { reader, name_table }
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
	Integral(usize)
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