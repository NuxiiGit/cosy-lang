use crate::diagnostics::*;
use crate::lexer::Lexer;
use crate::syntax::{
    token::*,
    ast::*
};

use std::iter::Peekable;

macro_rules! kind_of {
    ($($kind:pat),+, $(!$ignore:pat),*) => (|x| {
        match x.kind {
            $($ignore)|* => false,
            $($kind)|* => true,
            _ => false
        }
    });
    ($($kind:pat),+) => (|x| {
        match x.kind {
            $($kind)|* => true,
            _ => false
        }
    })
}

macro_rules! operator_of {
    ($($prefix:tt)*) => (|x| x.kind == TokenKind::Identifier(IdentifierKind::Operator) &&
            x.contains_prefix(&[$($prefix)*]))
}

/// Takes a lexer and uses it to construct a parse tree.
pub struct Parser<'a> {
    lexer : Peekable<Lexer<'a>>,
    errors : Vec<Error<'a>>
}
impl<'a> Parser<'a> {
    /// Creates a new parser from this scanner and converts it into a syntax tree.
    pub fn parse(lexer : Lexer<'a>) -> Result<'a> {
        let mut parser = Parser {
            lexer : lexer.peekable(),
            errors : Vec::new()
        };
        if let Some(prog) = parser.parse_prog() {
            Ok(prog)
        } else {
            Err(parser.errors)
        }
    }
    
    /// Parses a block statement.
    fn parse_prog(&mut self) -> Option<Prog<'a>> {
        let mut stmts = Vec::new();
        let mut invalidated = false;
        while !self.is_empty() && !self.satisfies(kind_of!(TokenKind::EoF)) {
            if let Some(stmt) = self.parse_stmt() {
                stmts.push(stmt);
            } else {
                self.synchronise();
                invalidated = true;
            }
        }
        if invalidated {
            None
        } else {
            Some(Prog { stmts })
        }
    }

    /// Parses any statement.
    fn parse_stmt(&mut self) -> Option<Stmt<'a>> {
        match self.peek() {
            Some(TokenKind::LeftBrace) => self.parse_stmt_block(),
            Some(TokenKind::Var) => self.parse_stmt_declr(),
            _ => self.parse_stmt_expr()
        }
    }

    /// Parses an expression statement.
    fn parse_stmt_expr(&mut self) -> Option<Stmt<'a>> {
        let expr = self.parse_expr()?;
        self.expects(kind_of!(TokenKind::SemiColon), "expected semicolon after expression statement")?;
        Some(Stmt::Expr { expr })
    }

    /// Parses an expression statement.
    fn parse_stmt_declr(&mut self) -> Option<Stmt<'a>> {
        self.expects(kind_of!(TokenKind::Var), "expected 'var' before declaration statement")?;
        let ident = self.expects(kind_of!(TokenKind::Identifier(..)), "expected identifier")?;
        self.expects(kind_of!(TokenKind::Assign), "expected '=' after left-hand-side of declaration expression")?;
        let expr = self.parse_expr()?;
        self.expects(kind_of!(TokenKind::SemiColon), "expected semicolon after declaration statement")?;
        Some(Stmt::Declr { ident, expr })
    }

    /// Parses an expression statement.
    fn parse_stmt_block(&mut self) -> Option<Stmt<'a>> {
        self.expects(kind_of!(TokenKind::LeftBrace), "expected opening '{' before block statement")?;
        let mut stmts = Vec::new();
        while !self.is_empty() && !self.satisfies(kind_of!(TokenKind::RightBrace)) {
            if let Some(stmt) = self.parse_stmt() {
                stmts.push(stmt);
            } else {
                self.synchronise();
            }
        }
        self.expects(kind_of!(TokenKind::RightBrace), "expected closing '}' after block statement")?;
        Some(Stmt::Block { stmts })
    }

    /// Parses any expression.
    fn parse_expr(&mut self) -> Option<Expr<'a>> {
        self.parse_expr_opblock()
    }

    /// Parses a stream of operator blocks given by any expression wrapped in backticks \`.
    fn parse_expr_opblock(&mut self) -> Option<Expr<'a>> {
        let mut expr = self.parse_expr_ops()?;
        while self.satisfies(kind_of!(TokenKind::Backtick)) {
            self.advance();
            let op = self.parse_expr_ops()?;
            self.expects(kind_of!(TokenKind::Backtick), "expected closing '`' in operator block")?;
            let right = self.parse_expr_ops()?;
            expr = Expr::binary_call(op, expr, right);
        }
        Some(expr)
    }

    /// Parses a stream of arbitrary operators.
    fn parse_expr_ops(&mut self) -> Option<Expr<'a>> {
        let mut expr = self.parse_expr_disjunction()?;
        while self.satisfies(kind_of!(TokenKind::Identifier(IdentifierKind::Operator))) {
            let ident = self.advance().unwrap();
            let right = self.parse_expr_disjunction()?;
            expr = Expr::binary_call(Expr::Variable { ident }, expr, right);
        }
        Some(expr)
    }

    /// Parses a stream of `|` and `^` binary operators.
    fn parse_expr_disjunction(&mut self) -> Option<Expr<'a>> {
        let mut expr = self.parse_expr_conjunction()?;
        while self.satisfies(operator_of!('|', '^')) {
            let ident = self.advance().unwrap();
            let right = self.parse_expr_conjunction()?;
            expr = Expr::binary_call(Expr::Variable { ident }, expr, right);
        }
        Some(expr)
    }

    /// Parses a stream of `&` binary operators.
    fn parse_expr_conjunction(&mut self) -> Option<Expr<'a>> {
        let mut expr = self.parse_expr_equality()?;
        while self.satisfies(operator_of!('&')) {
            let ident = self.advance().unwrap();
            let right = self.parse_expr_equality()?;
            expr = Expr::binary_call(Expr::Variable { ident }, expr, right);
        }
        Some(expr)
    }

    /// Parses a stream of `!=` and `==` binary operators.
    fn parse_expr_equality(&mut self) -> Option<Expr<'a>> {
        let mut expr = self.parse_expr_comparison()?;
        while self.satisfies(operator_of!('!', '=')) {
            let ident = self.advance().unwrap();
            let right = self.parse_expr_comparison()?;
            expr = Expr::binary_call(Expr::Variable { ident }, expr, right);
        }
        Some(expr)
    }

    /// Parses a stream of `>`, `<`, `>=`, and `<=` binary operators.
    fn parse_expr_comparison(&mut self) -> Option<Expr<'a>> {
        let mut expr = self.parse_expr_addition()?;
        while self.satisfies(operator_of!('<', '>')) {
            let ident = self.advance().unwrap();
            let right = self.parse_expr_addition()?;
            expr = Expr::binary_call(Expr::Variable { ident }, expr, right);
        }
        Some(expr)
    }

    /// Parses a stream of `+` and `-` binary operators.
    fn parse_expr_addition(&mut self) -> Option<Expr<'a>> {
        let mut expr = self.parse_expr_multiplication()?;
        while self.satisfies(operator_of!('+', '-')) {
            let ident = self.advance().unwrap();
            let right = self.parse_expr_multiplication()?;
            expr = Expr::binary_call(Expr::Variable { ident }, expr, right);
        }
        Some(expr)
    }

    /// Parses a stream of `*`, `/`, and `%` binary operators.
    fn parse_expr_multiplication(&mut self) -> Option<Expr<'a>> {
        let mut expr = self.parse_expr_call()?;
        while self.satisfies(operator_of!('*', '/', '%')) {
            let ident = self.advance().unwrap();
            let right = self.parse_expr_call()?;
            expr = Expr::binary_call(Expr::Variable { ident }, expr, right);
        }
        Some(expr)
    }

    /// Parses a stream of function calls.
    fn parse_expr_call(&mut self) -> Option<Expr<'a>> {
        let mut expr = self.parse_expr_member()?;
        while self.satisfies(kind_of!(
                TokenKind::Identifier(..),
                TokenKind::Literal(..),
                TokenKind::LeftParen,
                TokenKind::LeftBox,
                TokenKind::Backslash,
                !TokenKind::Identifier(IdentifierKind::Operator))) {
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
        while self.satisfies(kind_of!(TokenKind::Dot)) {
            self.advance();
            let ident = self.expects(kind_of!(TokenKind::Identifier(..)), "expected identifier after '.' symbol")?;
            expr = Expr::Member {
                expr : Box::new(expr),
                ident
            }
        }
        Some(expr)
    }

    /// Parses a lambda function.
    fn parse_expr_lambda(&mut self) -> Option<Expr<'a>> {
        if self.satisfies(kind_of!(TokenKind::Backslash)) {
            self.advance();
            let param = self.expects(kind_of!(TokenKind::Identifier(..)), "expected identifier after '\\' in lambda expression")?;
            if !self.satisfies(kind_of!(TokenKind::Backslash)) {
                self.expects(kind_of!(TokenKind::Arrow), "expected '->' after lambda expression parameter")?;
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
        if self.satisfies(kind_of!(TokenKind::Literal(..))) {
            let value = self.advance().unwrap();
            Some(Expr::Constant { value })
        } else if self.satisfies(kind_of!(TokenKind::Identifier(..))) {
            let ident = self.advance().unwrap();
            Some(Expr::Variable { ident })
        } else {
            self.parse_expr_tuple()
        }
    }

    /// Parses a tuple.
    fn parse_expr_tuple(&mut self) -> Option<Expr<'a>> {
        self.expects(kind_of!(TokenKind::LeftParen), "malformed expression")?;
        let mut exprs = vec![self.parse_expr()?];
        while self.satisfies(kind_of!(TokenKind::Comma)) {
            self.advance();
            let expr = self.parse_expr()?;
            exprs.push(expr);
        }
        self.expects(kind_of!(TokenKind::RightParen), "expected closing ')' after expression")?;
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
            if self.satisfies(kind_of!(TokenKind::SemiColon)) {
                self.advance();
                break;
            } else if self.satisfies(kind_of!(
                    TokenKind::Var,
                    TokenKind::Const,
                    TokenKind::If,
                    TokenKind::Unless,
                    TokenKind::Switch,
                    TokenKind::While,
                    TokenKind::Until,
                    TokenKind::Repeat,
                    TokenKind::For,
                    TokenKind::Function,
                    TokenKind::Object,
                    TokenKind::EoF)) {
                break;
            }
            self.advance();
        }
    }

    /// Advances the parser, but returns an error if some predicate isn't held.
    fn expects(&mut self, p : impl Fn(&Token) -> bool, on_err : &'static str) -> Option<Token<'a>> {
        if self.satisfies(p) {
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
    fn satisfies(&mut self, p : impl Fn(&Token) -> bool) -> bool {
        if let Some(Ok(token)) = self.lexer.peek() {
            p(token)
        } else {
            false
        }
    }

    /// Advances the parser.
    fn advance(&mut self) -> Option<Token<'a>> {
        match self.lexer.next() {
            Some(Ok(token)) => Some(token),
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

    /// Returns the next token kind.
    fn peek(&mut self) -> Option<&TokenKind> {
        if let Some(Ok(token)) = self.lexer.peek() {
            Some(&token.kind)
        } else {
            None
        }
    }
}

/// The result of the lexer.
pub type Result<'a> = std::result::Result<Prog<'a>, Vec<Error<'a>>>;