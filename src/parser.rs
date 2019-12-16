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
    previous : TokenKind,
    errors : Vec<Error<'a>>
}
impl<'a> Parser<'a> {
    /// Creates a new parser from this scanner.
    pub fn from(lexer : Lexer<'a>) -> Self {
        Parser {
            lexer : lexer.peekable(),
            previous : TokenKind::Unknown,
            errors : Vec::new()
        }
    }

    /// Consumes the parser and produces an abstract syntax tree.
    pub fn parse(mut self) -> Result<Prog<'a>, Vec<Error<'a>>> {
        let mut stmts = Vec::new();
        while !self.holds(|x| matches!(x, TokenKind::EoF)) {
            match self.parse_stmt() {
                Some(stmt) => stmts.push(stmt),
                None => {
                    self.synchronise();
                }
            }
        }
        if self.errors.is_empty() {
            Ok(Prog(stmts))
        } else {
            Err(self.errors)
        }
    }

    /// Parses any statement.
    fn parse_stmt(&mut self) -> Option<Stmt<'a>> {
        self.parse_stmt_expr()
    }

    /// Parses an expression statement.
    fn parse_stmt_expr(&mut self) -> Option<Stmt<'a>> {
        let expr = self.parse_expr()?;
        self.expects(|x| matches!(x, TokenKind::SemiColon), "expected semicolon after expression statement")?;
        Some(Stmt::Expr { expr })
    }

    /// Parses any expression.
    fn parse_expr(&mut self) -> Option<Expr<'a>> {
        self.parse_expr_opblock()
    }

    /// Parses a stream of operator blocks given by any expression wrapped in backticks \`.
    fn parse_expr_opblock(&mut self) -> Option<Expr<'a>> {
        let mut expr = self.parse_expr_ops()?;
        while self.holds(|x| matches!(x, TokenKind::Backtick)) {
            self.advance();
            let op = self.parse_expr_ops()?;
            self.expects(|x| matches!(x, TokenKind::Backtick), "expected closing '`' in operator block")?;
            let right = self.parse_expr_ops()?;
            expr = Expr::binary_call(op, expr, right);
        }
        Some(expr)
    }

    /// Parses a stream of arbitrary operators.
    fn parse_expr_ops(&mut self) -> Option<Expr<'a>> {
        let mut expr = self.parse_expr_disjunction()?;
        while self.holds(|x| matches!(x, TokenKind::Identifier(IdentifierKind::Operator))) {
            let ident = self.advance().unwrap();
            let right = self.parse_expr_disjunction()?;
            expr = Expr::binary_call(Expr::Variable { ident }, expr, right);
        }
        Some(expr)
    }

    /// Parses a stream of `|` and `^` binary operators.
    fn parse_expr_disjunction(&mut self) -> Option<Expr<'a>> {
        let mut expr = self.parse_expr_conjunction()?;
        while self.holds_content(|k, s| matches!(k, TokenKind::Identifier(IdentifierKind::Operator)) &&
                matches!(substr(s, 0, 1), "|" | "^")) {
            let ident = self.advance().unwrap();
            let right = self.parse_expr_conjunction()?;
            expr = Expr::binary_call(Expr::Variable { ident }, expr, right);
        }
        Some(expr)
    }

    /// Parses a stream of `&` binary operators.
    fn parse_expr_conjunction(&mut self) -> Option<Expr<'a>> {
        let mut expr = self.parse_expr_equality()?;
        while self.holds_content(|k, s| matches!(k, TokenKind::Identifier(IdentifierKind::Operator)) &&
                matches!(substr(s, 0, 1), "&")) {
            let ident = self.advance().unwrap();
            let right = self.parse_expr_equality()?;
            expr = Expr::binary_call(Expr::Variable { ident }, expr, right);
        }
        Some(expr)
    }

    /// Parses a stream of `!=` and `==` binary operators.
    fn parse_expr_equality(&mut self) -> Option<Expr<'a>> {
        let mut expr = self.parse_expr_comparison()?;
        while self.holds_content(|k, s| matches!(k, TokenKind::Identifier(IdentifierKind::Operator)) &&
                matches!(substr(s, 0, 1), "!" | "=")) {
            let ident = self.advance().unwrap();
            let right = self.parse_expr_comparison()?;
            expr = Expr::binary_call(Expr::Variable { ident }, expr, right);
        }
        Some(expr)
    }

    /// Parses a stream of `>`, `<`, `>=`, and `<=` binary operators.
    fn parse_expr_comparison(&mut self) -> Option<Expr<'a>> {
        let mut expr = self.parse_expr_addition()?;
        while self.holds_content(|k, s| matches!(k, TokenKind::Identifier(IdentifierKind::Operator)) &&
                matches!(substr(s, 0, 1), "<" | ">")) {
            let ident = self.advance().unwrap();
            let right = self.parse_expr_addition()?;
            expr = Expr::binary_call(Expr::Variable { ident }, expr, right);
        }
        Some(expr)
    }

    /// Parses a stream of `+` and `-` binary operators.
    fn parse_expr_addition(&mut self) -> Option<Expr<'a>> {
        let mut expr = self.parse_expr_multiplication()?;
        while self.holds_content(|k, s| matches!(k, TokenKind::Identifier(IdentifierKind::Operator)) &&
                matches!(substr(s, 0, 1), "+" | "-")) {
            let ident = self.advance().unwrap();
            let right = self.parse_expr_multiplication()?;
            expr = Expr::binary_call(Expr::Variable { ident }, expr, right);
        }
        Some(expr)
    }

    /// Parses a stream of `*`, `/`, and `%` binary operators.
    fn parse_expr_multiplication(&mut self) -> Option<Expr<'a>> {
        let mut expr = self.parse_expr_call()?;
        while self.holds_content(|k, s| matches!(k, TokenKind::Identifier(IdentifierKind::Operator)) &&
                matches!(substr(s, 0, 1), "*" | "/" | "%")) {
            let ident = self.advance().unwrap();
            let right = self.parse_expr_call()?;
            expr = Expr::binary_call(Expr::Variable { ident }, expr, right);
        }
        Some(expr)
    }

    /// Parses a stream of function calls.
    fn parse_expr_call(&mut self) -> Option<Expr<'a>> {
        let mut expr = self.parse_expr_member()?;
        while self.holds(|x| matches!(x,
                TokenKind::Literal(..) |
                TokenKind::Identifier(IdentifierKind::Alphanumeric) |
                TokenKind::LeftParen |
                TokenKind::Empty |
                TokenKind::LeftBox |
                TokenKind::Backslash)) {
            let arg = self.parse_expr_member()?;
            expr = Expr::Call {
                func : Box::new(expr),
                arg : Box::new(arg)
            }
        }
        Some(expr)
    }

    /// Parses a stream of member accesses.
    fn parse_expr_member(&mut self) -> Option<Expr<'a>> {
        let mut expr = self.parse_expr_lambda()?;
        while self.holds(|x| matches!(x, TokenKind::Dot)) {
            self.advance();
            let ident = self.expects(|x| matches!(x, TokenKind::Identifier(..)), "expected identifier after '.' symbol")?;
            expr = Expr::Member {
                expr : Box::new(expr),
                ident
            }
        }
        Some(expr)
    }

    /// Parses a lambda function.
    fn parse_expr_lambda(&mut self) -> Option<Expr<'a>> {
        if self.holds(|x| matches!(x, TokenKind::Backslash)) {
            self.advance();
            let param = self.expects(|x| matches!(x, TokenKind::Identifier(..)), "expected identifier after '\\' in lambda expression")?;
            if !self.holds(|x| matches!(x, TokenKind::Backslash)) {
                self.expects(|x| matches!(x, TokenKind::Arrow), "expected '->' after lambda expression parameter")?;
            }
            let body = self.parse_expr()?;
            Some(Expr::Lambda {
                param,
                body : Box::new(body)
            })
        } else {
            self.parse_expr_frontier()
        }
    }

    /// Parses expression literals and identifiers.
    fn parse_expr_frontier(&mut self) -> Option<Expr<'a>> {
        if self.holds(|x| matches!(x,
                TokenKind::Literal(..) | TokenKind::Empty)) {
            let value = self.advance().unwrap();
            Some(Expr::Constant { value })
        } else if self.holds(|x| matches!(x, TokenKind::Identifier(..))) {
            let ident = self.advance().unwrap();
            Some(Expr::Variable { ident })
        } else {
            self.parse_expr_tuple()
        }
    }

    /// Parses a tuple.
    fn parse_expr_tuple(&mut self) -> Option<Expr<'a>> {
        self.expects(|x| matches!(x, TokenKind::LeftParen), "malformed expression")?;
        let mut exprs = vec![self.parse_expr()?];
        while self.holds(|x| matches!(x, TokenKind::Comma)) {
            self.advance();
            let expr = self.parse_expr()?;
            exprs.push(expr);
        }
        self.expects(|x| matches!(x, TokenKind::RightParen), "expected closing ')' after expression")?;
        if exprs.len() == 1 {
            // singleton grouping
            Some(exprs.pop().unwrap())
        } else {
            Some(Expr::Tuple { exprs })
        }
    }

    /// Advances the parser until a stable line is found.
    fn synchronise(&mut self) {
        while !self.is_empty() {
            if self.previous == TokenKind::SemiColon {
                break;
            }
            self.advance();
        }
    }

    /// Advances the parser, but returns an error if some predicate isn't held.
    fn expects(&mut self, p : fn(&TokenKind) -> bool, on_err : &'static str) -> Option<Token<'a>> {
        if self.holds(p) {
            self.advance()
        } else {
            let token = self.advance()?;
            self.report(Error {
                reason : on_err,
                token
            });
            None
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

    /// Advances the parser.
    fn advance(&mut self) -> Option<Token<'a>> {
        match self.lexer.next() {
            Some(Ok(token)) => {
                self.previous = token.kind.clone(); // keep track of the previous token kind for error recovery
                Some(token)
            },
            Some(Err(e)) => {
                self.report(e);
                None
            },
            _ => None
        }
    }

    /// Reports an error.
    fn report(&mut self, e : Error<'a>) {
        self.errors.push(e);
    }

    /// Returns whether the lexer is empty.
    fn is_empty(&mut self) -> bool {
        if let None = self.lexer.peek() {
            true
        } else {
            false
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