pub mod scanner;

use scanner::{ CharReader, CharKind };

use super::ident::{ NameTable, Identifier };

use crate::common::diagnostics::SourcePosition;

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

/// Represents available token types.
#[derive(PartialEq, Debug, Clone)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    SemiColon,
    Literal(LiteralKind),
    Let,
    Assign,
    Identifier(Identifier, IdentifierKind),
    EoF,
    Unknown
}
impl TokenKind {
    /// Returns whether this token is a valid terminal value.
    pub fn is_terminal(&self) -> bool {
        self.is_literal() || self.is_identifier()
    }

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

    /// Returns `true` if the token is an operator identifier.
    pub fn is_operator(&self) -> bool {
        self.is_identifier() && !self.is_alphanumeric()
    }

    /// Returns `true` if the token is the end of the file.
    pub fn is_eof(&self) -> bool {
        matches!(self, Self::EoF)
    }
}

/// Converts a string into individual tokens.
pub struct Lexer<'a> {
    reader : CharReader<'a>,
    name_table : NameTable<'a>
}
impl Lexer<'_> {
    /// Returns the cursor of the lexer.
    pub fn cursor(&self) -> SourcePosition {
        self.reader.cursor()
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
                        CharKind::Bar => IdentifierKind::Or,
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
                        "let" => TokenKind::Let,
                        "=" => TokenKind::Assign,
                        _ => {
                            let slice = self.reader.slice();
                            TokenKind::Identifier(self.name_table.add(slice), kind)
                        }
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
impl<'a> From<&'a String> for Lexer<'a> {
    fn from(src : &'a String) -> Self {
        let slice : &str = &src;
        Self::from(slice)
    }
}
impl<'a> From<&'a str> for Lexer<'a> {
    fn from(src : &'a str) -> Self {
        let reader = CharReader::from(src);
        let name_table = NameTable::new();
        Self { reader, name_table }
    }
}

