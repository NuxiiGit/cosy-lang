pub mod scanner;

use crate::diagnostics::error::{ Error, Session };
use crate::syntax::token::*;

use scanner::Scanner;

/// An iterator over a string slice which produces `Token`s.
pub struct Lexer<'a, 'b> {
    sess : &'a mut Session,
    scanner : Scanner<'b>
}
impl<'a, 'b> Lexer<'a, 'b> {
    /// Creates a new lexer from this string scanner and parser session.
    pub fn from(sess : &'a mut Session, scanner : Scanner<'b>) -> Self {
        Lexer { sess, scanner }
    }

    /// Returns the next token.
    pub fn next(&mut self) -> Token {
        self.scanner.clear();
        let kind = if self.scanner.eof() {
            TokenKind::EoF
        } else {
            self.error("unknown token kind");
            self.scanner.advance();
            return self.next();
        };
        self.tokenise(kind)
    }

    /// Reports an error of this kind.
    pub fn error(&mut self, reason : &'static str) {
        let span = self.scanner.span();
        let token = Token {
            kind : TokenKind::Unknown,
            span
        };
        self.sess.report(Error { reason, token });
    }

    /// Creates a token of this kind from the scanner.
    pub fn tokenise(&mut self, kind : TokenKind) -> Token {
        let span = self.scanner.span();
        Token { kind, span }
    }
}