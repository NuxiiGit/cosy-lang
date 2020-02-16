pub mod scanner;

use libcosyc_diagnostics::{ Error, ErrorKind };
use libcosyc_syntax::token::*;

use scanner::{ Cursor, CharKind };

pub struct Lexer<'a> {
    scanner : Cursor<'a>,
    state : LexerState
}
impl<'a> Lexer<'a> {
    /// Creates a new lexer from this string scanner.
    pub fn from(scanner : Cursor<'a>) -> Self {
        Self {
            scanner,
            state : LexerState::Default
        }
    }

    /// Tokenises the current token and returns it.
    pub fn next(&mut self) -> Result<'a> {
        'search:
        loop {
            self.scanner.clear();
            let next = self.scanner.next();
            let peek = self.scanner.peek();
            let kind = match next {
                x if x.is_valid_whitespace() => {
                    while self.scanner.peek()
                            .is_valid_whitespace() {
                        self.scanner.next();
                    }
                    continue 'search;
                },
                CharKind::ForwardSlash if peek == CharKind::ForwardSlash => {
                    // line comments
                    while !self.scanner.peek()
                            .is_valid_ending() {
                        self.scanner.next();
                    }
                    continue 'search;
                },
                CharKind::ForwardSlash if peek == CharKind::Asterisk => {
                    // block comments
                    self.scanner.next();
                    let mut nests = 1;
                    loop {
                        let next = self.scanner.next();
                        let peek = self.scanner.peek();
                        match (next, peek) {
                            (_, CharKind::EoF) => {
                                return Err(self.make_error(ErrorKind::Warning, "unterminated block comment"));
                            },
                            (CharKind::ForwardSlash, CharKind::Asterisk) => {
                                self.scanner.next();
                                nests += 1
                            },
                            (CharKind::Asterisk, CharKind::ForwardSlash) => {
                                self.scanner.next();
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
                    while self.scanner.peek()
                            .is_valid_digit() {
                        self.scanner.next();
                    }
                    TokenKind::Literal(LiteralKind::Integer)
                },
                x if x.is_valid_graphic() => {
                    while self.scanner.peek()
                            .is_valid_graphic() {
                        self.scanner.next();
                    }
                    match self.scanner.substr() {
                        "var" => TokenKind::Keyword(KeywordKind::Var),
                        "if" => TokenKind::Keyword(KeywordKind::If),
                        "else" => TokenKind::Keyword(KeywordKind::Else),
                        _ => TokenKind::Identifier
                    }
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
                    while self.scanner.peek()
                            .is_valid_operator() {
                        self.scanner.next();
                    }
                    match self.scanner.substr() {
                        _ => TokenKind::Operator(kind)
                    }
                },
                CharKind::LeftParen => TokenKind::Symbol(SymbolKind::LeftParen),
                CharKind::RightParen => TokenKind::Symbol(SymbolKind::RightParen),
                CharKind::LeftBrace => TokenKind::Symbol(SymbolKind::LeftBrace),
                CharKind::RightBrace => TokenKind::Symbol(SymbolKind::RightBrace),
                CharKind::SemiColon => TokenKind::Symbol(SymbolKind::SemiColon),
                CharKind::Dollar => TokenKind::Symbol(SymbolKind::Dollar),
                CharKind::Backtick => TokenKind::Symbol(SymbolKind::Backtick),
                CharKind::Hashtag => {
                    if let CharKind::Graphic = self.scanner.peek() {
                        self.scanner.next();
                        while let CharKind::Graphic = self.scanner.peek() {
                            self.scanner.next();
                        }
                        TokenKind::Directive
                    } else {
                        return Err(self.make_error(ErrorKind::NonFatal, "expected graphic after hashtag symbol"));
                    }
                },
                CharKind::Address => TokenKind::Symbol(SymbolKind::Address),
                CharKind::EoF => TokenKind::EoF,
                _ => {
                    return Err(self.make_error(ErrorKind::NonFatal, "unknown symbol"));
                }
            };
            return Ok(self.make_token(kind));
        }
    }

    /// Creates a new error of this kind and reason.
    pub fn make_error(&self, kind : ErrorKind, reason : &'static str) -> Error<'a> {
        let token = self.make_token(TokenKind::Unknown);
        Error { reason, token, kind }
    }

    /// Creates a new token of this kind.
    pub fn make_token(&self, kind : TokenKind) -> Token<'a> {
        let context = self.scanner.context();
        Token { context, kind }
    }
}

/// The state of the lexer. This is used to parse strings as character arrays.
enum LexerState {
    Default
}

pub type Result<'a> = std::result::Result<Token<'a>, Error<'a>>;