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
        self.parse_expr()
    }

    /// Parses any expression.
    fn parse_expr(&mut self) -> Result<Expr<'a>, Error<'a>> {
        self.parse_expr_call()
    }

    /// Parses a stream of function calls.
    fn parse_expr_call(&mut self) -> Result<Expr<'a>, Error<'a>> {
        let mut expr = self.parse_expr_member()?;
        while self.holds(|x| x.starts_expr()) {
            let arg = self.parse_expr_member()?;
            expr = Expr::Call {
                func : Box::new(expr),
                arg : Box::new(arg)
            }
        }
        Ok(expr)
    }

    /// Parses a stream of member accesses.
    fn parse_expr_member(&mut self) -> Result<Expr<'a>, Error<'a>> {
        let mut expr = self.parse_expr_frontier()?;
        while self.holds(|x| matches!(x, TokenKind::Dot)) {
            self.token();
            let ident = self.expects(|x| matches!(x, TokenKind::Identifier(..)), "expected identifier after '.' symbol")?;
            expr = Expr::Member {
                expr : Box::new(expr),
                ident
            }
        }
        Ok(expr)
    }

    /// Parses expression literals and identifiers.
    fn parse_expr_frontier(&mut self) -> Result<Expr<'a>, Error<'a>> {
        if self.holds(|x| matches!(x, TokenKind::Literal(..))) {
            let value = self.token();
            Ok(Expr::Literal { value })
        } else if self.holds(|x| matches!(x, TokenKind::Identifier(..))) {
            let ident = self.token();
            Ok(Expr::Variable { ident })
        } else {
            self.parse_expr_tuple()
        }
    }

    /// Parses a tuple.
    fn parse_expr_tuple(&mut self) -> Result<Expr<'a>, Error<'a>> {
        self.expects(|x| matches!(x, TokenKind::LeftParen), "malformed expression")?;
        let mut exprs = vec![self.parse_expr()?];
        while self.holds(|x| matches!(x, TokenKind::Comma)) {
            self.token();
            let expr = self.parse_expr()?;
            exprs.push(expr);
        }
        self.expects(|x| matches!(x, TokenKind::RightParen), "expected closing ')' after expression")?;
        if exprs.len() == 1 {
            // singleton grouping
            Ok(exprs.pop().unwrap())
        } else {
            Ok(Expr::Tuple {
                exprs
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

    /// Advances the parser and returns the next token.
    /// # Panics
    /// Panics when there is an unhandled error.
    fn token(&mut self) -> Token<'a> {
        self.advance().unwrap()
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