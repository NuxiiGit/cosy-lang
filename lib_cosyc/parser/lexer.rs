use crate::scanner::{ Scanner, CharKind };
use crate::span::Span;

/// A struct which converts a stream of characters into individual tokens.
pub struct Lexer<'a> {
	reader : Scanner<'a>,
}
impl<'a> Lexer<'a> {
	/// Creates a new lexer from this source code.
	pub fn from(src : &'a str) -> Self {
		Self {
			reader : Scanner::from(src)
		}
	}

	/// Returns the next token in the source.
	pub fn next(&mut self) -> TokenKind {
		// skip preceeding whitespace
		self.reader.advance_while(CharKind::is_valid_whitespace);
		self.reader.clear_substr();
		match self.reader.next() {
			// individual symbols
			CharKind::Equals if !self.reader.peek().is_valid_operator() => TokenKind::Assign,
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
				self.reader.advance_until(CharKind::is_valid_ending);
				TokenKind::Comment {
					documentation : false
				}
			},
			// block comments
			CharKind::LeftDashedBrace => {
				let mut depth : u8 = 1;
				while depth >= 1 && depth < 255 {
					match self.reader.next() {
						CharKind::LeftDashedBrace => depth += 1,
						CharKind::RightDashedBrace => depth -= 1,
						CharKind::EoF => break,
						_ => ()
					}
				}
				TokenKind::BlockComment {
					poorly_braced : depth >= 1,
					exceeds_depth_limit : depth == 255
				}
			},
			// number literals
			x if x.is_valid_digit() => {
				self.reader.advance_while(CharKind::is_valid_digit);
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
						{ self.reader.peek() } else { &x };
				if template.is_valid_graphic() {
					self.reader.advance_while(CharKind::is_valid_graphic);
				} else if template.is_valid_operator() {
					self.reader.advance_while(CharKind::is_valid_operator);
				}
				loop {
					// all identifiers can end with any number of `'` (called "prime")
					self.reader.advance_while(|x| matches!(x, CharKind::SingleQuote));
					// join alphanumeric identifiers and operators with underscores
					if matches!(self.reader.peek(), CharKind::Underscore) {
						self.reader.advance_while(|x| matches!(x, CharKind::Underscore));
						let peeked = self.reader.peek();
						if peeked.is_valid_graphic() {
							self.reader.advance_while(CharKind::is_valid_graphic);
						} else if peeked.is_valid_operator() {
							self.reader.advance_while(CharKind::is_valid_operator);
						} else {
							break;
						}
					} else {
						break;
					}
				}
				// match substring for keywords
				match self.reader.substr() {
					"var" => TokenKind::Var,
					"if" => TokenKind::If,
					"else" => TokenKind::Else,
					_ => TokenKind::Identifier(kind)
				}
			}

			// alphabetic identifiers
			x if x.is_valid_graphic() => {
				self.reader.advance_while(CharKind::is_valid_graphic);
				TokenKind::Identifier(IdentifierKind::Alphabetic)
			},
			// operator identifiers
			x if x.is_valid_operator() => {
				let kind = match x {
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
				self.reader.advance_while(CharKind::is_valid_operator);
				if matches!(self.reader.peek(), CharKind::Underscore) {
					// operator followed by an alphanumeric identifier
					self.reader.next();
					self.reader.advance_while(CharKind::is_valid_graphic);
				}
				TokenKind::Identifier(kind)
			}
			// unknown symbol
			_ => TokenKind::Unknown
		}
	}

	/// Returns the span of the previously returned `Result`.
	pub fn span(&self) -> &Span {
		self.reader.span()
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
	Comment {
		documentation : bool
	},
	BlockComment {
		poorly_braced : bool,
		exceeds_depth_limit : bool
	},
	Unknown
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
	Character,
	Integer,
	Real
}