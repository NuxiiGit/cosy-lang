use crate::diagnostics::*;
use crate::lexer::Lexer;
use crate::syntax::{
    token::*,
    ast::*
};

use std::iter::Peekable;
use std::fmt;
use std::error;

/// Takes a lexer and uses it to construct a parse tree.
pub struct Parser<'a> {
    lexer : Peekable<Lexer<'a>>,
}
impl<'a> Parser<'a> {
    /// Creates a new parser from this scanner.
    pub fn from(lexer : Lexer<'a>) -> Self {
        Parser {
            lexer : lexer.peekable()
        }
    }

    /// Consumes the parser and produces an abstract syntax tree.
    pub fn parse(mut self) -> Result<Expr<'a>, Error<'a>> {
        unimplemented!()
    }

    /// Only advances the parser if the condition is met.
    fn matches(&mut self, kind : TokenKind) -> Result<Option<Token<'a>>, Error<'a>> {
        let consume = match self.lexer.peek() {
            Some(Ok(token)) => token.kind == kind,
            Some(Err(..)) => true,
            _ => unreachable!()
        };
        if consume {
            let token = self.advance()?;
            Ok(Some(token))
        } else {
            Ok(None)
        }
    }

    /// Advances the parser.
    fn advance(&mut self) -> Result<Token<'a>, Error<'a>> {
        match self.lexer.next() {
            Some(Ok(token)) => Ok(token),
            Some(Err(e)) => Err(e),
            _ => unreachable!()
        }
    }
}