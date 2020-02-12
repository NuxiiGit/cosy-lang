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
            let kind = match self.scanner.next() {
                CharKind::Whitespace => {
                    while let CharKind::Whitespace = self.scanner.peek() {
                        self.scanner.next();
                    }
                    continue 'search;
                },
                CharKind::Digit => {
                    while let
                    | x@CharKind::Digit
                    | x@CharKind::Underscore = self.scanner.peek() {
                        if let CharKind::Underscore = x {
                            self.scanner.skip();
                        } else {
                            self.scanner.next();
                        }
                    }
                    TokenKind::Literal(LiteralKind::Integer)
                },
                | CharKind::Graphic
                | CharKind::Underscore => {
                    while let
                    | CharKind::Graphic
                    | CharKind::Underscore
                    | CharKind::SingleQuote = self.scanner.peek() {
                        self.scanner.next();
                    }
                    match self.scanner.substr() {
                        "if" => TokenKind::Keyword(KeywordKind::If),
                        "else" => TokenKind::Keyword(KeywordKind::Else),
                        "then" => TokenKind::Keyword(KeywordKind::Then),
                        "var" => TokenKind::Keyword(KeywordKind::Var),
                        _ => TokenKind::Identifier(IdentifierKind::AlphaNumeric)
                    }
                },
                CharKind::LeftParen => TokenKind::Symbol(SymbolKind::LeftParen),
                CharKind::RightParen => TokenKind::Symbol(SymbolKind::RightParen),
                CharKind::LeftBrace => TokenKind::Symbol(SymbolKind::LeftBrace),
                CharKind::RightBrace => TokenKind::Symbol(SymbolKind::RightBrace),
                CharKind::LeftBox => continue 'search,
                CharKind::RightBox => continue 'search,
                CharKind::Dot => continue 'search,
                CharKind::Comma => continue 'search,
                CharKind::Colon => continue 'search,
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
                        self.error(ErrorKind::Fatal, "expected graphic after hashtag symbol");
                        continue 'search;
                    }
                },
                CharKind::Address => TokenKind::Symbol(SymbolKind::Address),
                CharKind::DoubleQuote => continue 'search,
                CharKind::SingleQuote => continue 'search,
                CharKind::Operator => {
                    while let CharKind::Operator = self.scanner.peek() {
                        self.scanner.next();
                    }
                    match self.scanner.substr() {
                        "//" => {
                            loop {
                                if let
                                | CharKind::NewLine
                                | CharKind::EoF = self.scanner.peek() {
                                    continue 'search;
                                }
                                self.scanner.skip();
                            }
                        }
                        _ => TokenKind::Identifier(IdentifierKind::Other)
                    }
                },
                CharKind::NewLine => continue 'search,
                CharKind::EoF => TokenKind::EoF
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

//use scanner::Scanner;
/*
/// An iterator over a string slice which produces `Token`s.
pub struct Lexer<'a, 'b> {
    scanner : Scanner<'a>,
    issues : &'b mut IssueTracker,
    state : LexerState
}
impl<'a, 'b> Lexer<'a, 'b> {
    /// Creates a new tokeniser from this string scanner.
    pub fn from(scanner : Scanner<'a>, issues : &'b mut IssueTracker) -> Self {
        Self {
            scanner,
            issues,
            state : LexerState::Default
        }
    }

    /// Tokenises the current token and returns it.
    pub fn next(&mut self) -> Token {
        'search:
        loop {
            self.scanner.clear();
            let kind = if let Some(current) = self.scanner.advance() {
                let next = self.scanner.chr();
                match self.state {
                    LexerState::Default => {
                        if current.is_whitespace() {
                            // ignore whitespace
                            self.scanner.advance_while(char::is_whitespace);
                            continue 'search;
                        } else if '/' == current && Some(&'/') == next {
                            // ignore line comments
                            self.scanner.advance_while(|x| x != '\n');
                            self.scanner.advance(); // ignore final `'\n'`
                            continue 'search;
                        } else if '/' == current && Some(&'*') == next {
                            // ignore block comments
                            self.scanner.advance();
                            let mut nests = 1;
                            loop {
                                match self.scanner.advance() {
                                    Some('*') if Some(&'/') == self.scanner.chr() => {
                                        self.scanner.advance();
                                        if nests == 1 {
                                            break;
                                        } else {
                                            nests -= 1;
                                        }
                                    },
                                    Some('/') if Some(&'*') == self.scanner.chr() => {
                                        self.scanner.advance();
                                        nests += 1
                                    },
                                    Some(_) => {},
                                    None => {
                                        self.error(ErrorKind::Warning, "unterminated block comment");
                                        break;
                                    }
                                }
                            }
                            continue 'search;
                        } else if current.is_ascii_digit() {
                            // tokenise numbers
                            let mut is_real = false;
                            while let Some(x) = self.scanner.chr() {
                                if let '.' = x {
                                    if is_real {
                                        break;
                                    } else {
                                        is_real = true;
                                    }
                                } else if !x.is_ascii_digit() {
                                    break;
                                }
                                self.scanner.advance();
                            }
                            TokenKind::Literal(if is_real {
                                LiteralKind::Real
                            } else {
                                LiteralKind::Integer
                            })
                        } else if current == '_' || current.is_alphanumeric() {
                            // tokenise keywords and identifiers
                            self.scanner.advance_while(|x| x == '_' || x == '\'' || x.is_alphanumeric());
                            match self.scanner.substr() {
                                "var" => TokenKind::Keyword(KeywordKind::Var),
                                "if" => TokenKind::Keyword(KeywordKind::If),
                                "else" => TokenKind::Keyword(KeywordKind::Else),
                                "then" => TokenKind::Keyword(KeywordKind::Then),
                                _ => TokenKind::Identifier(IdentifierKind::AlphaNumeric)
                            }
                        } else {
                            // tokenise symbols and basic operators
                            match current {
                                '(' => TokenKind::Symbol(SymbolKind::LeftParen),
                                ')' => TokenKind::Symbol(SymbolKind::RightParen),
                                '{' => TokenKind::Symbol(SymbolKind::LeftBrace),
                                '}' => TokenKind::Symbol(SymbolKind::RightBrace),
                                ';' => TokenKind::Symbol(SymbolKind::SemiColon),
                                '$' => TokenKind::Symbol(SymbolKind::Dollar),
                                '`' => TokenKind::Symbol(SymbolKind::Backtick),
                                '@' => TokenKind::Symbol(SymbolKind::Address),
                                '|' | '¦' => TokenKind::Identifier(IdentifierKind::Bar),
                                '^' => TokenKind::Identifier(IdentifierKind::Caret),
                                '&' => TokenKind::Identifier(IdentifierKind::Ampersand),
                                '!' => TokenKind::Identifier(IdentifierKind::Bang),
                                '=' => TokenKind::Identifier(IdentifierKind::Equals),
                                '<' => TokenKind::Identifier(IdentifierKind::LessThan),
                                '>' => TokenKind::Identifier(IdentifierKind::GreaterThan),
                                '+' => TokenKind::Identifier(IdentifierKind::Plus),
                                '-' => TokenKind::Identifier(IdentifierKind::Minus),
                                '*' => TokenKind::Identifier(IdentifierKind::Asterisk),
                                '/' => TokenKind::Identifier(IdentifierKind::ForwardSlash),
                                '\\' => TokenKind::Identifier(IdentifierKind::BackSlash),
                                '%' => TokenKind::Identifier(IdentifierKind::Percent),
                                _ => TokenKind::Identifier(IdentifierKind::Other)
                            }
                        }
                    }
                }
            } else {
                TokenKind::EoF
            };
            break self.scanner.tokenise(kind);
        }
    }

    /// Reports a new error with this reason.
    pub fn error(&mut self, kind : ErrorKind, reason : &'static str) {
        let token = self.scanner.tokenise(TokenKind::Unknown);
        self.issues.report(Error { reason, token, kind });
    }
}

/// The current lexer state. Is used to parse strings as character arrays.
enum LexerState {
    Default
}*/