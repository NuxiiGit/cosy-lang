use super::source_pos::Span;
use super::scanner::Lexer;
use super::syntax::{
    token::*,
    ast::*
};

use std::iter::Peekable;
use std::fmt;
use std::error;

/// Takes a lexer and uses it to construct a parse tree.
pub struct Parser<'a> {
    lexer : Peekable<Lexer<'a>>,
    eof : bool
}
impl<'a> Parser<'a> {
    /// Creates a new parser from this scanner.
    pub fn from(lexer : Lexer<'a>) -> Self {
        Parser {
            lexer : lexer.peekable(),
            eof : false
        }
    }

    /// Consumes the parser and produces an abstract syntax tree.
    pub fn parse(mut self) -> Result<Expr<'a>, ParseError<'a>> {
        unimplemented!()
    }

    /// Only advances the parser if the condition is met.
    fn matches(&mut self, kind : TokenKind) -> Result<Option<Token<'a>>, ParseError<'a>> {
        if let Some(token) = self.lexer.peek() {
            if if let TokenKind::Err(..) = token.kind {
                true
            } else {
                token.kind == kind
            } {
                Ok(Some(self.advance()?))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    /// Advances the parser.
    fn advance(&mut self) -> Result<Token<'a>, ParseError<'a>> {
        if let Some(token) = self.lexer.next() {
            let Token { kind, .. } = &token;
            if let TokenKind::Err(msg) = kind {
                Err(ParseError {
                    reason : msg,
                    token : Some(token)
                })
            } else {
                Ok(token)
            }
        } else {
            self.eof = true;
            Err(ParseError {
                reason : "unexpected end of file",
                token : None
            })
        }
    }
}

/// A struct which stores error information.
#[derive(Debug)]
pub struct ParseError<'a> {
    pub reason : &'static str,
    pub token : Option<Token<'a>>
}
impl fmt::Display for ParseError<'_> {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        if let Some(Token { span, .. }) = &self.token {
            write!(out, "ParseError! {}: {}", span, self.reason)
        } else {
            write!(out, "ParseError! {}", self.reason)
        }
    }
}
impl error::Error for ParseError<'_> {}