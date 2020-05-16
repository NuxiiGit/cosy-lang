use crate::scanner::{ CharReader, CharKind };
use crate::span::Span;

use std::mem;

/// Converts a string into individual tokens.
pub struct Lexer<'a> {
	reader : CharReader<'a>,
	peeked : TokenKind,
}
impl<'a> From<&'a str> for Lexer<'a> {
	fn from(src : &'a str) -> Self {
		let mut reader = CharReader::from(src);
		let peeked = reader.tokenise();
		Self { reader, peeked }
	}
}

impl CharReader<'_> {
	/// Returns the next token in the source.
	pub fn tokenise(&mut self) -> TokenKind {
		unimplemented!()
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