pub mod scanner;

use scanner::{ CharReader, CharKind };

use libcosyc_span::Span;

/// Represents literal types.
#[derive(PartialEq, Debug, Clone)]
pub enum LiteralKind {
    Integral
}

/// Represents identifier types.
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

/// Represents token types.
#[derive(PartialEq, Debug, Clone)]
pub enum TokenKind {
    Literal(LiteralKind),
    Identifier(IdentifierKind),
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    SemiColon,
    Let,
    Assign,
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
        matches!(self, Self::Identifier(.., IdentifierKind::Alphanumeric))
    }

    /// Returns whether this token is a valid terminal value.
    pub fn is_terminal(&self) -> bool {
        self.is_literal() || self.is_identifier()
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

/// Converts a stream of characters into lexemes, ignoring whitespace.
pub struct Lexer<'a> {
    reader : CharReader<'a>
}
impl Lexer<'_> {
    /// Returns the span of the current lexeme.
    pub fn span(&self) -> &Span {
        self.reader.span()
    }

    /// Returns the next token of the source.
    pub fn generate_token(&mut self) -> TokenKind {
    'search:
        loop {
            self.reader.reset_span();
            let kind = match self.reader.advance() {
                // whitestuff
                CharKind::Whitestuff => {
                    self.reader.advance_while(|x| matches!(x, CharKind::Whitestuff));
                    continue 'search;
                },
                // symbols
                CharKind::LeftParen => TokenKind::LeftParen,
                CharKind::RightParen => TokenKind::RightParen,
                CharKind::SemiColon => TokenKind::SemiColon,
                // end of file
                CharKind::EoF => TokenKind::EoF,
                // unknown symbol
                _ => TokenKind::Unknown
            };
            break kind;
        }
    }
}
impl<'a> From<&'a str> for Lexer<'a> {
    fn from(src : &'a str) -> Self {
        let reader = CharReader::from(src);
        Self { reader }
    }
}
