use crate::span::Span;

use std::str::CharIndices;

pub struct CharReader<'a> {
	src : &'a str,
	chars : CharIndices<'a>,
	current : CharKind,
	span : Span
}
impl<'a> CharReader<'a> {
	/// Creates a new character scanner from this string.
	pub fn from(src : &'a str) -> Self {
		let mut chars = src.char_indices();
		let current = chars
				.next()
				.map(|(_, snd)| CharKind::identify(snd))
				.unwrap_or(CharKind::EoF);
		Self {
			src,
			chars,
			current,
			span : Span::new()
		}
	}
}

/// Represents various kinds of character types.
#[derive(PartialEq, Debug, Clone)]
pub enum CharKind {
	Cr,
	Lf,
	CrLf,
	Tab,
	Space,
	Digit,
	Graphic,
	Underscore,
	LeftParen,
	RightParen,
	LeftBrace,
	RightBrace,
	LeftBox,
	RightBox,
	Dot,
	Comma,
	Colon,
	SemiColon,
	Dollar,
	Backtick,
	Hashtag,
	Address,
	DoubleQuote,
	SingleQuote,
	Bar,
	Caret,
	Ampersand,
	Bang,
	Hook,
	Equals,
	LessThan,
	GreaterThan,
	Plus,
	Minus,
	Tilde,
	Asterisk,
	Solidus,
	ReverseSolidus,
	Percent,
	/// Any other unicode character or symbol.
	Other,
	EoF
}
impl CharKind {
	/// Converts a character into its respective `CharKind`.
	pub fn identify(c : char) -> CharKind {
		use CharKind::*;
		match c {
			'\r' => Cr,
			'\n' => Lf,
			'\t' => Tab,
			x if x.is_whitespace() => Space,
			x if x.is_ascii_digit() => Digit,
			x if x.is_alphanumeric() => Graphic,
			'_' => Underscore,
			'(' => LeftParen,
			')' => RightParen,
			'{' => LeftBrace,
			'}' => RightBrace,
			'[' => LeftBox,
			']' => RightBox,
			'.' => Dot,
			',' => Comma,
			':' => Colon,
			';' => SemiColon,
			'$' => Dollar,
			'`' => Backtick,
			'#' => Hashtag,
			'@' => Address,
			'"' => DoubleQuote,
			'\'' => SingleQuote,
			'|' | '¦' => Bar,
			'^' => Caret,
			'&' => Ampersand,
			'!' => Bang,
			'?' => Hook,
			'=' => Equals,
			'<' => LessThan,
			'>' => GreaterThan,
			'+' => Plus,
			'-' => Minus,
			'~' => Tilde,
			'*' => Asterisk,
			'/' => Solidus,
			'\\' => ReverseSolidus,
			'%' => Percent,
			_ => Other
		}
	}

	/// Returns whether the char is valid whitespace.
	pub fn is_valid_whitespace(&self) -> bool {
		use CharKind::*;
		matches!(self, Tab | Space) || self.is_valid_newline()
	}

	/// Returns whether the char is a valid line ending.
	pub fn is_valid_ending(&self) -> bool {
		matches!(self, CharKind::EoF) || self.is_valid_newline()
	}

	/// Returns whether the char is valid new line character.
	pub fn is_valid_newline(&self) -> bool {
		use CharKind::*;
		matches!(self, Cr | Lf | CrLf)
	}

	/// Returns whether the char is a valid graphic.
	pub fn is_valid_graphic(&self) -> bool {
		matches!(self, CharKind::Graphic) || self.is_valid_digit()
	}

	/// Returns whether the char is a valid digit.
	pub fn is_valid_digit(&self) -> bool {
		matches!(self, CharKind::Digit)
	}

	/// Returns whether the char is a valid operator.
	pub fn is_valid_operator(&self) -> bool {
		use CharKind::*;
		matches!(self
				, Dot
				| Colon
				| Dollar
				| Hashtag
				| Address
				| Bar
				| Caret
				| Ampersand
				| Bang
				| Hook
				| Equals
				| LessThan
				| GreaterThan
				| Plus
				| Minus
				| Tilde
				| Asterisk
				| Solidus
				| ReverseSolidus
				| Percent
				| Other)
	}
}