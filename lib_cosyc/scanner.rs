use crate::span::Span;

use std::str::CharIndices;
use std::mem;

/// A struct which converts a script into individual character spans.
pub struct Scanner<'a> {
	src : &'a str,
	chars : CharIndices<'a>,
	previous : CharKind,
	current : CharKind,
	cursor : usize,
	span : Span
}
impl<'a> Scanner<'a> {
	/// Creates a new parser session from this source code.
	pub fn from(src : &'a str) -> Self {
		let mut chars = src.char_indices();
		let current = chars
				.next()
				.map(|(_, x)| CharKind::identify(x))
				.unwrap_or(CharKind::EoF);
		Self {
			src,
			chars,
			previous : CharKind::BoF,
			current,
			cursor : 0,
			span : Span {
				line : 1,
				begin : 0,
				end : 0
			}
		}
	}

	/// Advances the scanner whilst some predicate holds.
	/// Potentially dangerous if the `EoF` character always satisfies your predicate.
	pub fn advance_while(&mut self, f : fn(&CharKind) -> bool) {
		while f(self.peek()) {
			self.next();
		}
	}

	/// Advances the scanner until some predicate holds.
	/// Potentially dangerous if the `EoF` character token does not satisfy your predicate.
	pub fn advance_until(&mut self, f : fn(&CharKind) -> bool) {
		while !f(self.peek()) {
			self.next();
		}
	}

	/// Returns the current peeked character.
	pub fn peek(&self) -> &CharKind {
		&self.previous
	}

	/// Advances the session scanner.
	pub fn next(&mut self) -> CharKind {
		if self.previous.is_valid_newline() {
			self.span.line += 1;
		}
		self.span.end = self.cursor;
		let next = if let Some((i, c)) = self.chars.next() {
			self.cursor = i;
			CharKind::identify(c)
		} else {
			self.cursor = self.src.len();
			CharKind::EoF
		};
		let option = match (&self.current, &next) {
			(CharKind::Minus, CharKind::Minus) => Some(CharKind::DoubleDash),
			(CharKind::LeftBrace, CharKind::Minus) => Some(CharKind::LeftBrash),
			(CharKind::Minus, CharKind::RightBrace) => Some(CharKind::RightBrash),
			(CharKind::Minus, CharKind::GreaterThan) => Some(CharKind::RightArrow),
			(CharKind::LessThan, CharKind::Minus) => Some(CharKind::LeftArrow),
			(CharKind::Equals, CharKind::GreaterThan) => Some(CharKind::RightImply),
			(CharKind::LessThan, CharKind::Equals) => Some(CharKind::LeftImply),
			(CharKind::Colon, CharKind::Colon) => Some(CharKind::SquaredFourDots),
			(CharKind::Cr, CharKind::Lf) => Some(CharKind::CrLf),
			_ => None
		};
		let current = if let Some(kind) = option {
			let (i, current) = self.chars
				.next()
				.map(|(i, x)| (i, CharKind::identify(x)))
				.unwrap_or((self.src.len(), CharKind::EoF));
			self.cursor = i;
			self.current = current;
			kind
		} else {
			mem::replace(&mut self.current, next)
		};
		mem::replace(&mut self.previous, current)
	}

	/// Returns the current substring.
	pub fn substr(&self) -> &'a str {
		&self.src[self.span.begin..self.span.end]
	}

	/// Clears the current substring.
	pub fn clear_substr(&mut self) {
		self.span.begin = self.span.end;
	}
	
	/// Returns the current span.
	pub fn span(&self) -> &Span {
		&self.span
	}
}

/// An enum which stores character kinds.
#[derive(PartialEq, Debug, Clone)]
pub enum CharKind {
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
	ForwardSlash,
	BackSlash,
	Percent,
	/// Double dash, `--`.
	DoubleDash,
	/// Left brace followed by a minus sign, `{-`.
	LeftBrash,
	/// Right brace following a minus sign, `-}`.
	RightBrash,
	/// Left arrow, specifically `←`.
	LeftArrow,
	/// Right arrow, specifically `→`.
	RightArrow,
	/// Left implication, specifically `⇐`.
	LeftImply,
	/// Right implication, specifically `⇒`.
	RightImply,
	/// Double colon, specifically `⸬`.
	SquaredFourDots,
	/// Carriage return, `\r`.
	Cr,
	/// Line feed, `\n`.
	Lf,
	/// Used to make sure windows files dont advance two lines per new line.
	CrLf,
	/// Any other white space or control character.
	Space,
	/// Beginning of the file.
	/// Specifically used to help detect `CrLf`.
	BoF,
	/// End of the file.
	/// Specifically used when there are no more characters in the stream.
	EoF,
	/// Any other unicode character or symbol.
	Other
}
impl CharKind {
	/// Converts a character into its respective `CharKind`.
	pub fn identify(c : char) -> CharKind {
		use CharKind::*;
		match c {
			'\r' => Cr,
			'\n' => Lf,
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
			| '|'
			| '¦' => Bar,
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
			'/' => ForwardSlash,
			'\\' => BackSlash,
			'%' => Percent,
			'←' => LeftArrow,
			'→' => RightArrow,
			'⇐' => LeftImply,
			'⇒' => RightImply,
			'⸬' => SquaredFourDots,
			_ => Other
		}
	}

	/// Returns whether the char is valid whitespace.
	pub fn is_valid_whitespace(&self) -> bool {
		use CharKind::*;
		if let
		| Space
		| BoF = self {
			true
		} else {
			self.is_valid_newline()
		}
	}

	/// Returns whether the char is a valid line ending.
	pub fn is_valid_ending(&self) -> bool {
		if let CharKind::EoF = self {
			true
		} else {
			self.is_valid_newline()
		}
	}

	/// Returns whether the char is valid new line character.
	pub fn is_valid_newline(&self) -> bool {
		use CharKind::*;
		if let
		| Cr
		| Lf
		| CrLf = self
				{ true } else { false }
	}

	/// Returns whether the char is a valid digit.
	pub fn is_valid_digit(&self) -> bool {
		if let CharKind::Digit = self
				{ true } else { false }
	}

	/// Returns whether the char is a valid graphic.
	pub fn is_valid_graphic(&self) -> bool {
		use CharKind::*;
		if let
		| Graphic
		| Underscore
		| SingleQuote = self
				{ true } else { false }
	}

	/// Returns whether the char is a valid operator.
	pub fn is_valid_operator(&self) -> bool {
		use CharKind::*;
		if let
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
		| ForwardSlash
		| BackSlash
		| Percent
		| Other = self
				{ true } else { false }
	}
}