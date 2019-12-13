use crate::diagnostics::*;
use crate::lexer::Lexer;
use crate::syntax::{
    token::*,
    ast::*
};

use std::iter::Peekable;

macro_rules! matches {
    ($value:expr, $($pattern:tt)*) => ({
        match $value {
            $($pattern)* => true,
            _ => false
        }
    });
}

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
        self.parse_expr_frontier()
    }

    fn parse_expr_frontier(&mut self) -> Result<Expr<'a>, Error<'a>> {
        if self.holds(|x| matches!(x, TokenKind::Literal(..))) {
            let value = self.advance().unwrap();
            Ok(Expr::Literal { value })
        } else if self.holds(|x| matches!(x, TokenKind::Identifier(..))) {
            let ident = self.advance().unwrap();
            Ok(Expr::Variable { ident })
        } else {
            // malformed expression
            let token = self.advance()?;
            Err(Error {
                reason : "malformed expression",
                kind : ErrorKind::Fatal,
                token
            })
        }
    }

    /// Advances the parser, but returns an error if some predicate isn't held.
    fn expects(&mut self, p : fn(&TokenKind) -> bool, on_err : &'static str) -> Result<Token<'a>, Error<'a>> {
        if self.holds(p) {
            self.advance()
        } else {
            let token = self.advance()?;
            Err(Error {
                reason : on_err,
                kind : ErrorKind::Fatal,
                token
            })
        }
    }

    /// Returns `true` if the next token satisfies some predicate.
    fn holds(&mut self, p : fn(&TokenKind) -> bool) -> bool {
        if let Some(Ok(token)) = self.lexer.peek() {
            p(&token.kind)
        } else {
            false
        }
    }

    /// Advances the parser.
    /// # Panics
    /// Panics when there is an unexpected end to the lexer.
    fn advance(&mut self) -> Result<Token<'a>, Error<'a>> {
        match self.lexer.next() {
            Some(Ok(token)) => Ok(token),
            Some(Err(e)) => Err(e),
            _ => unreachable!()
        }
    }
}