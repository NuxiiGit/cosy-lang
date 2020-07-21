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
    Graphic,
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
    pub fn is_graphic(&self) -> bool {
        matches!(self, Self::Identifier(.., IdentifierKind::Graphic))
    }

    /// Returns whether this token is a valid terminal value.
    pub fn is_terminal(&self) -> bool {
        self.is_literal() || self.is_identifier()
    }

    /// Returns `true` if the token is an operator identifier.
    pub fn is_operator(&self) -> bool {
        self.is_identifier() && !self.is_graphic()
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

    fn read_digit_identifier(&mut self) {
        self.reader.advance_while(CharKind::is_valid_digit);
    }

    fn read_alphabetic_identifier(&mut self) {
        self.reader.advance_while(CharKind::is_valid_graphic);
    }

    fn read_operator_identifier(&mut self) {
        self.reader.advance_while(CharKind::is_valid_operator);
    }

    fn identifier_separator_exists(&mut self) -> bool {
        if matches!(self.reader.current(), CharKind::Underscore) {
            self.reader.advance_while(|x| matches!(x, CharKind::Underscore));
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
                CharKind::Whitestuff => {
                    self.reader.advance_while(|x| matches!(x, CharKind::Whitestuff));
                    continue 'search;
                },
                // symbols
                CharKind::LeftParen => TokenKind::LeftParen,
                CharKind::RightParen => TokenKind::RightParen,
                CharKind::SemiColon => TokenKind::SemiColon,
                // numbers
                x if x.is_valid_digit() => {
                    self.read_digit_identifier();
                    if self.identifier_separator_exists() {
                        self.read_identifier();
                        TokenKind::Identifier(IdentifierKind::Graphic)
                    } else {
                        TokenKind::Literal(LiteralKind::Integral)
                    }
                },
                // alphabetic
                x if x.is_valid_graphic() => {
                    self.read_alphabetic_identifier();
                    TokenKind::Identifier(IdentifierKind::Graphic)
                },
                // operator
                x if x.is_valid_operator() => {
                    self.read_operator_identifier();
                    TokenKind::Identifier(match x {
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
                    })
                },
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
