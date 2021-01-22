pub mod symbol;
pub mod token;
pub mod reader;

use reader::SymbolReader;
use symbol::SymbolKind;
use token::TokenKind;
use libcosyc_diagnostic::source::Span;

/// Converts a string slice into lexemes, ignoring whitespace.
pub struct Lexer<'a> {
    reader : SymbolReader<'a>,
    ignore_next_symbol : bool
}

impl<'a> Lexer<'a> {
    /// Returns the span of the current lexeme.
    pub fn span(&self) -> &Span {
        self.reader.span()
    }

    /// Returns the substring of the current lexeme.
    pub fn substring(&self) -> &'a str {
        self.reader.substring()
    }

    /// Returns the next token of the source.
    pub fn generate_token(&mut self) -> TokenKind {
        if self.ignore_next_symbol {
            self.reader.advance();
            self.ignore_next_symbol = false;
        }
        self.reader.reset_span();
        match self.reader.advance() {
            x if x.is_valid_whitespace() => {
                self.reader.advance_while(SymbolKind::is_valid_whitespace);
                TokenKind::Whitestuff
            },
            SymbolKind::LeftParen => TokenKind::LeftParen,
            SymbolKind::RightParen => TokenKind::RightParen,
            SymbolKind::Colon => TokenKind::Colon,
            SymbolKind::Pound => TokenKind::Pound,
            SymbolKind::Plus => TokenKind::Plus,
            SymbolKind::Minus => {
                match self.reader.peek() {
                    SymbolKind::Minus => {
                        self.reader.advance_while(|x| !x.is_valid_terminator());
                        TokenKind::Comment
                    },
                    _ => TokenKind::Minus
                }
            },
            SymbolKind::LessThan => {
                match self.reader.peek() {
                    SymbolKind::Bar => {
                        self.reader.advance();
                        TokenKind::LeftPipe
                    },
                    _ => TokenKind::Unknown
                }
            },
            SymbolKind::Bar => {
                match self.reader.peek() {
                    SymbolKind::GreaterThan => {
                        self.reader.advance();
                        TokenKind::RightPipe
                    },
                    _ => TokenKind::Unknown
                }
            },
            x if x.is_valid_digit() => {
                self.reader.advance_while(SymbolKind::is_valid_digit);
                TokenKind::Integral
            },
            x if x.is_valid_graphic() => {
                self.reader.advance_while(SymbolKind::is_valid_graphic);
                // alphabetic identifiers can end with any number of `'` (called "prime")
                self.reader.advance_while(|x| matches!(x, SymbolKind::SingleQuote));
                match self.substring() {
                    "_" => TokenKind::Hole,
                    "let" => TokenKind::Let,
                    "i8" => TokenKind::I8,
                    "type" => TokenKind::Type,
                    _ => TokenKind::Identifier
                }
            },
            SymbolKind::Backtick => {
                self.reader.reset_span(); // this is used so that identifiers Foo and `Foo` are the same
                self.reader.advance_while(|x| !matches!(x, SymbolKind::Backtick | SymbolKind::EoL));
                let closed = matches!(self.reader.peek(), SymbolKind::Backtick);
                self.ignore_next_symbol = closed;
                TokenKind::RawIdentifier { closed }
            }
            SymbolKind::EoF => TokenKind::EoF,
            _ => TokenKind::Unknown
        }
    }
}

impl<'a> From<SymbolReader<'a>> for Lexer<'a> {
    fn from(reader : SymbolReader<'a>) -> Self {
        let ignore_next_symbol = false;
        Self { reader, ignore_next_symbol }
    }
}

impl<'a> From<&'a str> for Lexer<'a> {
    fn from(src : &'a str) -> Self {
        Lexer::from(SymbolReader::from(src))
    }
}

impl Into<Span> for Lexer<'_> {
    fn into(mut self) -> Span {
        self.reader.advance_while(|x| !matches!(x, SymbolKind::EoF));
        self.span().clone()
    }
}
