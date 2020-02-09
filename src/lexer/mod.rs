pub mod scanner;

use crate::diagnostics::IssueTracker;
use crate::diagnostics::error::{ Error, ErrorKind };
use crate::syntax::token::*;

use scanner::Scanner;

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
}