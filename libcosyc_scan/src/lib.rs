pub mod symbol;
pub mod token;
pub mod reader;

use reader::SymbolReader;
use symbol::SymbolKind;
use token::{ TokenKind, LiteralKind, GraphicKind, IdentifierKind };
use libcosyc_diagnostic::source::Span;

/// Converts a string slice into lexemes, ignoring whitespace.
pub struct Lexer<'a> {
    reader : SymbolReader<'a>
}

impl Lexer<'_> {
    /// Returns the span of the current lexeme.
    pub fn span(&self) -> &Span {
        self.reader.span()
    }

    fn read_digit_identifier(&mut self) {
        self.reader.advance_while(|x| x.is_valid_digit() || matches!(x, SymbolKind::SingleQuote));
    }

    fn read_alphabetic_identifier(&mut self) {
        self.reader.advance_while(SymbolKind::is_valid_graphic);
        // alphabetic identifiers can end with any number of `'` (called "prime")
        self.reader.advance_while(|x| matches!(x, SymbolKind::SingleQuote));
    }

    fn read_operator_identifier(&mut self) {
        self.reader.advance_while(SymbolKind::is_valid_operator);
    }

    fn identifier_separator_exists(&mut self) -> bool {
        if matches!(self.reader.current(), SymbolKind::Underscore) {
            self.reader.advance_while(|x| matches!(x, SymbolKind::Underscore));
            true
        } else {
            false
        }
    }

    fn read_identifier(&mut self) {
        match self.reader.current() {
            x if x.is_valid_digit() => self.read_digit_identifier(),
            x if x.is_valid_graphic() => self.read_alphabetic_identifier(),
            x if x.is_valid_operator() => self.read_operator_identifier(),
            _ => return // complete
        }
        if self.identifier_separator_exists() {
            self.read_identifier();
        }
    }

    /// Returns the next token of the source.
    pub fn generate_token(&mut self) -> TokenKind {
    'search:
        loop {
            self.reader.reset_span();
            let kind = match self.reader.advance() {
                // whitestuff
                SymbolKind::Whitestuff => {
                    self.reader.advance_while(|x| matches!(x, SymbolKind::Whitestuff));
                    continue 'search;
                },
                // symbols
                SymbolKind::LeftParen => TokenKind::LeftParen,
                SymbolKind::RightParen => TokenKind::RightParen,
                // numbers
                x if x.is_valid_digit() => {
                    self.read_digit_identifier();
                    if self.identifier_separator_exists() {
                        self.read_identifier();
                        TokenKind::Identifier(
                                IdentifierKind::Graphic(GraphicKind::Other))
                    } else {
                        TokenKind::Literal(LiteralKind::Integral)
                    }
                },
                // alphabetic
                x if x.is_valid_graphic() => {
                    self.read_alphabetic_identifier();
                    if self.identifier_separator_exists() {
                        self.read_identifier();
                    }
                    let kind = match self.reader.substring() {
                        "let" => GraphicKind::Let,
                        _ => GraphicKind::Other
                    };
                    TokenKind::Identifier(IdentifierKind::Graphic(kind))
                },
                // operator
                x if x.is_valid_operator() => {
                    self.read_operator_identifier();
                    if self.identifier_separator_exists() {
                        self.read_identifier();
                    }
                    let kind = match x {
                        SymbolKind::Plus => IdentifierKind::Addition,
                        _ => IdentifierKind::Other
                    };
                    TokenKind::Identifier(kind)
                },
                // underscore
                SymbolKind::Underscore => {
                    self.identifier_separator_exists();
                    self.read_identifier();
                    let kind = if let "_" = self.reader.substring() {
                        GraphicKind::Hole
                    } else {
                        GraphicKind::IgnoreMe
                    };
                    TokenKind::Identifier(IdentifierKind::Graphic(kind))
                }
                // end of file
                SymbolKind::EoF => TokenKind::EoF,
                // unknown symbol
                _ => TokenKind::Unknown
            };
            break kind;
        }
    }
}

impl<'a> From<SymbolReader<'a>> for Lexer<'a> {
    fn from(reader : SymbolReader<'a>) -> Self {
        Self { reader }
    }
}

impl<'a> From<&'a str> for Lexer<'a> {
    fn from(src : &'a str) -> Self {
        Lexer::from(SymbolReader::from(src))
    }
}

