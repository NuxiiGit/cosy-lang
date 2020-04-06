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
	pub fn next(&mut self) -> Result {
		'search: loop {
			self.reader.clear_substr();
			let kind = match self.reader.next() {
				x if x.is_valid_whitespace() => {
					self.reader.advance_while(CharKind::is_valid_whitespace);
					continue 'search
				},
				CharKind::DoubleDash => {
					// line comments
					self.reader.advance_until(CharKind::is_valid_ending);
					continue 'search
				},
				CharKind::LeftDashedBrace => {
					// block comments
					let mut depth = 1;
					while depth >= 1 {
						match self.reader.next() {
							CharKind::LeftDashedBrace => depth += 1,
							CharKind::RightDashedBrace => depth -= 1,
							CharKind::EoF => return Err("unterminated block comment"),
							_ => ()
						}
					}
					continue 'search
				}
				CharKind::EoF => TokenKind::EoF,
				_ => return Err("unexpected symbol")
			};
			break Ok(kind)
        }
	}

	/// Returns the substring of the previously returned `Result`.
	pub fn context(&self) -> &'a str {
		self.reader.substr()
	}

	/// Returns the span of the previously returned `Result`.
	pub fn span(&self) -> &Span {
		self.reader.span()
	}
}

/// The lexer result will return `Ok(TokenKind)` if successful, or `Err(&'static str)` if a lexer error occurs.
pub type Result = std::result::Result<TokenKind, &'static str>;

/// An enum which describes available token types.
#[derive(PartialEq, Debug, Clone)]
pub enum TokenKind {
	Keyword(KeywordKind),
	Symbol(SymbolKind),
	Identifier,
	Operator(OperatorKind),
	Value(ValueKind),
	EoF
}
impl TokenKind {
	/// Returns `true` if the token is a keyword.
	pub fn is_keyword(&self) -> bool {
		if let TokenKind::Keyword(..) = self
				{ true } else { false }
	}

	/// Returns `true` if the token is a symbol.
	pub fn is_symbol(&self) -> bool {
		if let TokenKind::Symbol(..) = self
				{ true } else { false }
	}

	/// Returns `true` if the token is an identifier.
	pub fn is_identifier(&self) -> bool {
		if let
		| TokenKind::Identifier
		| TokenKind::Operator(..) = self
				{ true } else { false }
	}

	/// Returns `true` if the token is an operator.
	pub fn is_operator(&self) -> bool {
		if let TokenKind::Operator(..) = self
				{ false } else { true }
	}

	/// Returns `true` if the token is a value.
	pub fn is_value(&self) -> bool {
		if let TokenKind::Value(..) = self
				{ true } else { false }
	}

	/// Returns `true` if the token is the end of the file.
	pub fn is_eof(&self) -> bool {
		if let TokenKind::EoF = self
				{ true } else { false }
	}

	/// Returns `true` if the token can start an expression.
	pub fn is_nonterminal(&self) -> bool {
		self.is_identifier()
				|| self.is_value()
				|| matches!(self,
						Self::Symbol(SymbolKind::LeftParen) |
						Self::Symbol(SymbolKind::Hashtag))
	}
}

/// An enum which describes available keyword types.
#[derive(PartialEq, Debug, Clone)]
pub enum KeywordKind {
	Var,
	If,
	Else
}

/// An enum which describes available symbol types.
#[derive(PartialEq, Debug, Clone)]
pub enum SymbolKind {
	LeftParen,
	RightParen,
	LeftBrace,
	RightBrace,
	SemiColon,
	Dollar,
	Backtick,
	Hashtag,
	Address
}

/// An enum which describes available identifier types.
#[derive(PartialEq, Debug, Clone)]
pub enum OperatorKind {
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
pub enum ValueKind {
	Character,
	Integer,
	Real
}