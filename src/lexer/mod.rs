pub mod scanner;

use crate::diagnostics::IssueTracker;
use crate::diagnostics::error::{ Error, ErrorKind };
use crate::syntax::token::*;

use scanner::Scanner;

/// An iterator over a string slice which produces `Token`s.
pub struct Lexer<'a, 'b> {
    scanner : Scanner<'a>,
    issues : &'b mut IssueTracker
}
impl<'a, 'b> Lexer<'a, 'b> {
    /// Creates a new tokeniser from this string scanner.
    pub fn from(scanner : Scanner<'a>, issues : &'b mut IssueTracker) -> Self {
        Self { scanner, issues }
    }

    /// Tokenises the current token and returns it.
    pub fn next(&mut self) -> Token {
        'search:
        loop {
            self.scanner.clear();
            let kind = if let Some(current) = self.scanner.advance() {
                let next = self.scanner.chr();
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
                }
                self.error(ErrorKind::Fatal, "not implemented");
                continue 'search;
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