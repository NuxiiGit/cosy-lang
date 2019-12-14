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
        self.parse_expr_opblock()
    }

    /// Parses a stream of operator blocks given by any expression wrapped in backticks \`.
    fn parse_expr_opblock(&mut self) -> Result<Expr<'a>, Error<'a>> {
        let mut expr = self.parse_expr_addition()?;
        while self.holds(|x| matches!(x, TokenKind::Backtick)) {
            self.consume();
            let op = self.parse_expr_addition()?;
            self.expects(|x| matches!(x, TokenKind::Backtick), "expected closing '`' in operator block")?;
            let right = self.parse_expr_addition()?;
            expr = Expr::binary_call(op, expr, right);
        }
        Ok(expr)
    }

    /// Parses a stream of `+` and `-` binary operators.
    fn parse_expr_addition(&mut self) -> Result<Expr<'a>, Error<'a>> {
        let mut expr = self.parse_expr_multiplication()?;
        while self.holds_content(|k, s| matches!(k, TokenKind::Identifier(IdentifierKind::Operator)) &&
                matches!(substr(s, 0, 1), "+" | "-")) {
            let ident = self.consume();
            let right = self.parse_expr_multiplication()?;
            expr = Expr::binary_call(Expr::Variable { ident }, expr, right);
        }
        Ok(expr)
    }

    /// Parses a stream of `*`, `/`, and `%` binary operators.
    fn parse_expr_multiplication(&mut self) -> Result<Expr<'a>, Error<'a>> {
        let mut expr = self.parse_expr_ops()?;
        while self.holds_content(|k, s| matches!(k, TokenKind::Identifier(IdentifierKind::Operator)) &&
                matches!(substr(s, 0, 1), "*" | "/" | "%")) {
            let ident = self.consume();
            let right = self.parse_expr_ops()?;
            expr = Expr::binary_call(Expr::Variable { ident }, expr, right);
        }
        Ok(expr)
    }

    /// Parses a stream of arbitrary operators.
    fn parse_expr_ops(&mut self) -> Result<Expr<'a>, Error<'a>> {
        let mut expr = self.parse_expr_call()?;
        while self.holds(|x| matches!(x, TokenKind::Identifier(IdentifierKind::Operator))) {
            let ident = self.consume();
            let right = self.parse_expr_call()?;
            expr = Expr::binary_call(Expr::Variable { ident }, expr, right);
        }
        Ok(expr)
    }

    /// Parses a stream of function calls.
    fn parse_expr_call(&mut self) -> Result<Expr<'a>, Error<'a>> {
        let mut expr = self.parse_expr_member()?;
        while self.holds(|x| matches!(x,
                TokenKind::Literal(..) |
                TokenKind::Identifier(IdentifierKind::Alphanumeric) |
                TokenKind::LeftParen |
                TokenKind::LeftBox)) {
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
            self.consume();
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
        if self.holds(|x| matches!(x,
                TokenKind::Literal(..) | TokenKind::Empty)) {
            let value = self.consume();
            Ok(Expr::Constant { value })
        } else if self.holds(|x| matches!(x, TokenKind::Identifier(..))) {
            let ident = self.consume();
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
            self.consume();
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

    /// Returns `true` if the next token satisfies some predicate.
    fn holds_content(&mut self, p : fn(&TokenKind, &str) -> bool) -> bool {
        if let Some(Ok(token)) = self.lexer.peek() {
            p(&token.kind, token.span.content)
        } else {
            false
        }
    }

    /// Advances the parser and returns the next token.
    /// # Panics
    /// Panics when there is an unhandled error.
    fn consume(&mut self) -> Token<'a> {
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

/// Returns a substring of this `str`.
fn substr<'a>(x : &'a str, start : usize, n : usize) -> &'a str {
    let end = if let Some((i, _)) = x
            .char_indices()
            .skip(n + start)
            .next() {
        i
    } else {
        x.len()
    };
    &x[start..end]
}