use libcosyc_syntax::{ Context, token::* };
use libcosyc_diagnostics::{ Error, ErrorKind };

use std::str::CharIndices;
use std::iter::Peekable;

pub struct Lexer<'a> {
    reader : StringReader<'a>,
}
impl<'a> Lexer<'a> {
    /// Creates a new lexer from this string reader.
    pub fn new(reader : StringReader<'a>) -> Self {
        Self { reader }
    }

    /// Creates a new error of this kind and reason.
    fn make_error(&self, reason : &'static str) -> Error<'a> {
        let token = self.make_token(TokenKind::Unknown);
        let kind = ErrorKind::Fatal;
        Error { reason, token, kind }
    }

    /// Creates a new token of this kind.
    fn make_token(&self, kind : TokenKind) -> Token<'a> {
        let context = self.reader.context();
        Token { context, kind }
    }
}
impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, Error<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        Some('search: loop {
            self.reader.reset_context();
            let next = self.reader.next();
            let peek = self.reader.peek();
            let result = match next {
                x if x.is_valid_whitespace() => {
                    while self.reader.peek()
                            .is_valid_whitespace() {
                        self.reader.next();
                    }
                    continue 'search;
                },
                CharKind::Minus if peek == CharKind::Minus => {
                    // line comments
                    while !self.reader.peek()
                            .is_valid_ending() {
                        self.reader.next();
                    }
                    continue 'search;
                },
                CharKind::LeftBrace if peek == CharKind::Minus => {
                    // block comments
                    self.reader.next();
                    let mut nests = 1;
                    loop {
                        let next = self.reader.next();
                        let peek = self.reader.peek();
                        match (next, peek) {
                            (_, CharKind::EoF) => {
                                break Err("unterminated block comment");
                            },
                            (CharKind::LeftBrace, CharKind::Minus) => {
                                self.reader.next();
                                nests += 1
                            },
                            (CharKind::Minus, CharKind::RightBrace) => {
                                self.reader.next();
                                if nests == 1 {
                                    continue 'search;
                                } else {
                                    nests -= 1;
                                }
                            },
                            _ => ()
                        }
                    }
                },
                x if x.is_valid_digit() => {
                    while self.reader.peek()
                            .is_valid_digit() {
                        self.reader.next();
                    }
                    Ok(TokenKind::Literal(LiteralKind::Integer))
                },
                x if x.is_valid_graphic() => {
                    while self.reader.peek()
                            .is_valid_graphic() {
                        self.reader.next();
                    }
                    Ok(match self.reader.substr() {
                        "var" => TokenKind::Keyword(KeywordKind::Var),
                        "if" => TokenKind::Keyword(KeywordKind::If),
                        "else" => TokenKind::Keyword(KeywordKind::Else),
                        _ => TokenKind::Identifier
                    })
                },
                x if x.is_valid_operator() => {
                    let kind = match x {
                        CharKind::Bar => OperatorKind::Bar,
                        CharKind::Caret => OperatorKind::Caret,
                        CharKind::Ampersand => OperatorKind::Ampersand,
                        CharKind::Bang => OperatorKind::Bang,
                        CharKind::Equals => OperatorKind::Equals,
                        CharKind::LessThan => OperatorKind::LessThan,
                        CharKind::GreaterThan => OperatorKind::GreaterThan,
                        CharKind::Plus => OperatorKind::Plus,
                        CharKind::Minus => OperatorKind::Minus,
                        CharKind::Asterisk => OperatorKind::Asterisk,
                        CharKind::ForwardSlash => OperatorKind::ForwardSlash,
                        CharKind::Percent => OperatorKind::Percent,
                        _ => OperatorKind::Other
                    };
                    while self.reader.peek()
                            .is_valid_operator() {
                        self.reader.next();
                    }
                    Ok(match self.reader.substr() {
                        _ => TokenKind::Operator(kind)
                    })
                },
                CharKind::LeftParen => Ok(TokenKind::Symbol(SymbolKind::LeftParen)),
                CharKind::RightParen => Ok(TokenKind::Symbol(SymbolKind::RightParen)),
                CharKind::LeftBrace => Ok(TokenKind::Symbol(SymbolKind::LeftBrace)),
                CharKind::RightBrace => Ok(TokenKind::Symbol(SymbolKind::RightBrace)),
                CharKind::SemiColon => Ok(TokenKind::Symbol(SymbolKind::SemiColon)),
                CharKind::Dollar => Ok(TokenKind::Symbol(SymbolKind::Dollar)),
                CharKind::Backtick => Ok(TokenKind::Symbol(SymbolKind::Backtick)),
                CharKind::Hashtag => {
                    if let CharKind::Graphic = self.reader.peek() {
                        self.reader.next();
                        while let CharKind::Graphic = self.reader.peek() {
                            self.reader.next();
                        }
                        Ok(TokenKind::Directive)
                    } else {
                        Err("expected graphic after hashtag symbol")
                    }
                },
                CharKind::Address => Ok(TokenKind::Symbol(SymbolKind::Address)),
                CharKind::EoF => Ok(TokenKind::EoF),
                _ => Err("unexpected symbol")
            };
            break 'search match result {
                Ok(kind) => Ok(self.make_token(kind)),
                Err(reason) => Err(self.make_error(reason))
            }
        })
    }
}

/// A structure over a string slice which produces individual `Context`s.
pub struct StringReader<'a> {
    src : &'a str,
    chars : Peekable<CharIndices<'a>>,
    line : usize,
    byte_start : usize,
    byte_end : usize
}
impl<'a> StringReader<'a> {
    /// Creates a new scanner from this source.
    pub fn from(src : &'a str) -> Self {
        Self {
            src,
            chars : src.char_indices().peekable(),
            line : 1,
            byte_start : 0,
            byte_end : 0
        }
    }

    /// Returns the kind of the next character.
    pub fn peek(&mut self) -> CharKind {
        if let Some((_, c)) = self.chars.peek() {
            CharKind::identify(c)
        } else {
            CharKind::EoF
        }
    }

    /// Advances the scanner.
    pub fn next(&mut self) -> CharKind {
        let kind = if let Some((_, c)) = self.chars.next() {
            CharKind::identify(&c)
        } else {
            CharKind::EoF
        };
        if let CharKind::NewLine = kind {
            self.line += 1;
        }
        if let Some((i, _)) = self.chars.peek() {
            self.byte_end = *i;
        } else {
            self.byte_end = self.src.len();
        }
        kind
    }

    /// Returns the current substring.
    pub fn substr(&self) -> &'a str {
        &self.src[self.byte_start..self.byte_end]
    }

    /// Clears the current substring.
    pub fn reset_context(&mut self) {
        self.byte_start = self.byte_end;
    }

    /// Returns the current context for the current substring.
    pub fn context(&self) -> Context<'a> {
        Context {
            src : self.substr(),
            line : self.line
        }
    }
}

/// An enum which stores character kinds.
#[derive(PartialEq, Debug, Clone)]
pub enum CharKind {
    NewLine,
    Whitespace,
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
    EoF,
    Other
}
impl CharKind {
    /// Converts a character into its respective `CharKind`.
    pub fn identify(c : &char) -> CharKind {
        match c {
            '\n' => CharKind::NewLine,
            x if x.is_whitespace() => CharKind::Whitespace,
            x if x.is_ascii_digit() => CharKind::Digit,
            x if x.is_alphanumeric() => CharKind::Graphic,
            '_' => CharKind::Underscore,
            '(' => CharKind::LeftParen,
            ')' => CharKind::RightParen,
            '{' => CharKind::LeftBrace,
            '}' => CharKind::RightBrace,
            '[' => CharKind::LeftBox,
            ']' => CharKind::RightBox,
            '.' => CharKind::Dot,
            ',' => CharKind::Comma,
            ':' => CharKind::Colon,
            ';' => CharKind::SemiColon,
            '$' => CharKind::Dollar,
            '`' => CharKind::Backtick,
            '#' => CharKind::Hashtag,
            '@' => CharKind::Address,
            '"' => CharKind::DoubleQuote,
            '\'' => CharKind::SingleQuote,
            | '|'
            | '¦' => CharKind::Bar,
            '^' => CharKind::Caret,
            '&' => CharKind::Ampersand,
            '!' => CharKind::Bang,
            '?' => CharKind::Hook,
            '=' => CharKind::Equals,
            '<' => CharKind::LessThan,
            '>' => CharKind::GreaterThan,
            '+' => CharKind::Plus,
            '-' => CharKind::Minus,
            '~' => CharKind::Tilde,
            '*' => CharKind::Asterisk,
            '/' => CharKind::ForwardSlash,
            '\\' => CharKind::BackSlash,
            '%' => CharKind::Percent,
            _ => CharKind::Other
        }
    }

    /// Returns whether the char is valid whitespace.
    pub fn is_valid_whitespace(&self) -> bool {
        if let
        | CharKind::NewLine
        | CharKind::Whitespace = self {
            true
        } else {
            false
        }
    }

    /// Returns whether the char is a valid digit.
    pub fn is_valid_digit(&self) -> bool {
        if let
        | CharKind::Digit = self {
            true
        } else {
            false
        }
    }

    /// Returns whether the char is a valid graphic.
    pub fn is_valid_graphic(&self) -> bool {
        if let
        | CharKind::Graphic
        | CharKind::Underscore
        | CharKind::SingleQuote = self {
            true
        } else {
            false
        }
    }

    /// Returns whether the char is a valid operator.
    pub fn is_valid_operator(&self) -> bool {
        if let
        | CharKind::Bar
        | CharKind::Caret
        | CharKind::Ampersand
        | CharKind::Bang
        | CharKind::Hook
        | CharKind::Equals
        | CharKind::LessThan
        | CharKind::GreaterThan
        | CharKind::Plus
        | CharKind::Minus
        | CharKind::Tilde
        | CharKind::Asterisk
        | CharKind::ForwardSlash
        | CharKind::BackSlash
        | CharKind::Percent
        | CharKind::Other = self {
            true
        } else {
            false
        }
    }

    /// Returns whether the char is a valid line ending.
    pub fn is_valid_ending(&self) -> bool {
        if let
        | CharKind::NewLine
        | CharKind::EoF = self {
            true
        } else {
            false
        }
    }
}