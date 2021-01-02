pub mod symbol;
pub mod token;
pub mod reader;

use reader::SymbolReader;
use symbol::SymbolKind;
use token::{ TokenKind, LiteralKind, IdentifierKind };
use libcosyc_diagnostic::source::Span;

/// Converts a string slice into lexemes, ignoring whitespace.
pub struct Lexer<'a> {
    reader : SymbolReader<'a>,
    ignore_next_symbol : bool
}

impl Lexer<'_> {
    /// Returns the span of the current lexeme.
    pub fn span(&self) -> &Span {
        self.reader.span()
    }

    /// Returns the next token of the source.
    pub fn generate_token(&mut self) -> TokenKind {
        if self.ignore_next_symbol {
            self.reader.advance();
            self.ignore_next_symbol = false;
        }
    'search:
        loop {
            self.reader.reset_span();
            let kind = match self.reader.advance() {
                SymbolKind::Whitestuff => {
                    self.reader.advance_while(SymbolKind::is_valid_whitespace);
                    continue 'search;
                },
                SymbolKind::Minus if
                        matches!(self.reader.peek(), SymbolKind::Minus) => {
                    self.reader.advance_while(|x| !x.is_valid_terminator());
                    TokenKind::Comment
                },
                SymbolKind::LeftParen => TokenKind::LeftParen,
                SymbolKind::RightParen => TokenKind::RightParen,
                SymbolKind::Plus => TokenKind::Plus,
                x if x.is_valid_digit() => {
                    self.reader.advance_while(SymbolKind::is_valid_digit);
                    TokenKind::Literal(LiteralKind::Integral)
                },
                x if x.is_valid_graphic() => {
                    self.reader.advance_while(SymbolKind::is_valid_graphic);
                    // alphabetic identifiers can end with any number of `'` (called "prime")
                    self.reader.advance_while(|x| matches!(x, SymbolKind::SingleQuote));
                    let kind = match self.reader.substring() {
                        "_" => IdentifierKind::Hole,
                        "let" => IdentifierKind::Let,
                        _ => IdentifierKind::Graphic
                    };
                    TokenKind::Identifier(kind)
                },
                SymbolKind::Backtick => {
                    self.reader.reset_span(); // this is used so that identifiers Foo and `Foo` are the same
                    self.reader.advance_while(|x| !matches!(x, SymbolKind::Backtick));
                    let closed = matches!(self.reader.peek(), SymbolKind::Backtick);
                    self.ignore_next_symbol = closed;
                    TokenKind::Identifier(IdentifierKind::Raw { closed })
                }
                SymbolKind::EoF => TokenKind::EoF,
                _ => TokenKind::Unknown
            };
            break kind;
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

