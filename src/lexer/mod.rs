pub mod scanner;

use crate::common::diagnostics::{ IssueTracker, Error, ErrorKind };
use crate::common::syntax::*;

use scanner::FileScanner;

pub struct Lexer<'a> {
    scanner : FileScanner,
    state : LexerState,
    issues : &'a mut IssueTracker
}
impl<'a> Lexer<'a> {
    /// Creates a new lexer from this file scanner.
    pub fn from(scanner : FileScanner, issues : &'a mut IssueTracker) -> Self {
        Self {
            scanner,
            state : LexerState::Default,
            issues
        }
    }

    /// Tokenises the current token and returns it.
    pub fn next(&mut self) -> Token {
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
                                self.error(ErrorKind::Warning, "unterminated block comment");
                                continue 'search;
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
                        self.error(ErrorKind::NonFatal, "expected graphic after hashtag symbol");
                        continue 'search;
                    }
                },
                CharKind::Address => TokenKind::Symbol(SymbolKind::Address),
                CharKind::EoF => TokenKind::EoF,
                _ => {
                    self.error(ErrorKind::NonFatal, "unknown symbol");
                    continue 'search;
                }
            };
            let context = self.scanner.context();
            break Token { context, kind };
        }
    }

    /// Reports a new error with this reason.
    pub fn error(&mut self, kind : ErrorKind, reason : &'static str) {
        let context = self.scanner.context();
        let token = Token {
            context,
            kind : TokenKind::Unknown
        };
        self.issues.report(Error { reason, token, kind });
    }
}

/// The state of the lexer. This is used to parse strings as character arrays.
enum LexerState {
    Default
}