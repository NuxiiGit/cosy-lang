pub mod syntax;

use libcosyc_diagnostic::{
    source::Span,
    error::{ IssueTracker, CompilerError }
};
use libcosyc_scan::{ Lexer, token::TokenKind };
use crate::syntax as ast;
use std::mem;

fn generate_token(lexer : &mut Lexer) -> TokenKind {
    loop {
        // ignore whitespace tokens
        let token = lexer.generate_token();
        if !matches!(token, TokenKind::Comment
                | TokenKind::Whitestuff) {
            break token;
        }
    }
}

/// Produces a concrete syntax tree from concrete syntax.
pub struct Parser<'a> {
    issues : &'a mut IssueTracker,
    lexer : Lexer<'a>,
    peeked : TokenKind,
    span_previous : Span
}

impl<'a> Parser<'a> {
    /// Creates a new parser from this lexer and issue tracker.
    pub fn new(mut lexer : Lexer<'a>, issues : &'a mut IssueTracker) -> Self {
        let peeked = generate_token(&mut lexer);
        let span_previous = Span::default();
        Self { issues, lexer, peeked, span_previous }
    }

    /// Returns the span of the current lexeme.
    pub fn span(&self) -> &Span {
        &self.span_previous
    }

    /// Returns the span of the peeked lexeme.
    pub fn span_peek(&self) -> &Span {
        self.lexer.span()
    }

    /// Returns whether the current token satisfies a predicate `p`.
    /// The function will always return `false` for the EoF token.
    pub fn sat(&self, p : fn(&TokenKind) -> bool) -> bool {
        match &self.peeked {
            TokenKind::EoF => false,
            x => p(x)
        }
    }

    /// Advances the parser and returns the the previous lexeme.
    pub fn advance(&mut self) -> TokenKind {
        self.span_previous = self.lexer.span().clone();
        let next = generate_token(&mut self.lexer);
        mem::replace(&mut self.peeked, next)
    }

    /// Reports an error to the issue tracker, using the current token span if no span exists.
    pub fn report(&mut self, mut error : CompilerError) {
        if !error.has_span() {
            error = error.span(self.span());
        }
        self.issues.report_error(error);
    }

    /// Returns the token if it satisfies the predicate `p`, otherwise the error is reported.
    pub fn expect(&mut self, p : fn(&TokenKind) -> bool, error : CompilerError) -> Option<TokenKind> {
        if self.sat(p) {
            Some(self.advance())
        } else {
            self.report(error);
            None
        }
    }

    /// Returns whether the parser contains additional unparsed tokens.
    pub fn is_empty(&self) -> bool {
        matches!(self.peeked, TokenKind::EoF)
    }

    /// Entry point for parsing any expression.
    pub fn parse_expr(&mut self) -> Option<ast::Expr> {
        self.parse_expr_infix()
    }

    /// Parses custom infix operators
    pub fn parse_expr_infix(&mut self) -> Option<ast::Expr> {
        let mut expr = self.parse_expr_addition()?;
        while self.sat(TokenKind::is_identifier) {
            let kind = ast::BinaryOpKind::Custom(
                    Box::new(self.parse_expr_terminal()?));
            let lexpr = Box::new(expr);
            let rexpr = Box::new(self.parse_expr_addition()?);
            let span = Span::new(lexpr.span.begin, rexpr.span.end);
            let kind = ast::ExprKind::BinaryOp { kind, lexpr, rexpr };
            expr = ast::Expr { span, kind };
        }
        Some(expr)
    }

    /// Parses `+` and `-` binary operators.
    pub fn parse_expr_addition(&mut self) -> Option<ast::Expr> {
        let mut expr = self.parse_expr_terminal()?;
        while self.sat(|x| matches!(x, TokenKind::Plus | TokenKind::Minus)) {
            let kind = match self.advance() {
                TokenKind::Plus => ast::BinaryOpKind::Add,
                TokenKind::Minus => ast::BinaryOpKind::Subtract,
                _ => {
                    self.report(CompilerError::bug()
                            .reason("invalid addition operator"));
                    return None;
                }
            };
            let lexpr = Box::new(expr);
            let rexpr = Box::new(self.parse_expr_terminal()?);
            let span = Span::new(lexpr.span.begin, rexpr.span.end);
            let kind = ast::ExprKind::BinaryOp { kind, lexpr, rexpr };
            expr = ast::Expr { span, kind };
        }
        Some(expr)
    }

    /// Parses literals and identifiers.
    pub fn parse_expr_terminal(&mut self) -> Option<ast::Expr> {
        if self.sat(TokenKind::is_terminal) {
            let kind = match self.advance() {
                x if x.is_identifier() => ast::ExprKind::Variable,
                TokenKind::Integral => ast::ExprKind::Integral,
                _ => {
                    self.report(CompilerError::bug()
                            .reason("invalid terminal kind"));
                    return None;
                }
            };
            let span = self.span().clone();
            Some(ast::Expr { span, kind })
        } else {
            self.parse_expr_grouping()
        }
    }

    /// Parses groupings of expressions.
    pub fn parse_expr_grouping(&mut self) -> Option<ast::Expr> {
        if self.sat(|x| matches!(x, TokenKind::LeftParen)) {
            self.advance();
            let expr = self.parse_expr()?;
            self.expect(|x| matches!(x, TokenKind::RightParen),
                    CompilerError::new()
                            .span(&expr.span)
                            .reason("expected closing `)` at the end of grouping")
                            .note("consider adding `)` after this expression"))?;
            Some(expr)
        } else {
            self.advance();
            self.report(CompilerError::new()
                    .reason("unknown synbol in expression")
                    .note("consider removing this token"));
            None
        }
    }
}

impl<'a> Into<Lexer<'a>> for Parser<'a> {
    fn into(self) -> Lexer<'a> {
        self.lexer
    }
}

/// Generates the AST of this source code and reports any errors to this `IssueTracker`.
pub fn build_ast(src : &str, issues : &mut IssueTracker) -> Option<ast::Expr> {
    let lexer = Lexer::from(src);
    let mut parser = Parser::new(lexer, issues);
    let program = parser.parse_expr()?;
    if parser.is_empty() {
        Some(program)
    } else {
        let lexer : Lexer = parser.into();
        let span : Span = lexer.into();
        issues.report_error(CompilerError::bug()
                .span(&span)
                .reason("unparsed tokens at the end of this file"));
        None
    }
}
