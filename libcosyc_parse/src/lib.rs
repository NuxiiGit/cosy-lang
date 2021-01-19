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
    pub fn report<T>(&mut self, mut error : CompilerError) -> Option<T> {
        if !error.has_span() {
            error = error.span(self.span());
        }
        self.issues.report_error(error);
        None
    }

    /// Returns the token if it satisfies the predicate `p`, otherwise the error is reported.
    pub fn expect(&mut self, p : fn(&TokenKind) -> bool, error : CompilerError) -> Option<TokenKind> {
        if self.sat(p) {
            Some(self.advance())
        } else {
            self.report(error)
        }
    }

    /// Returns whether the parser contains additional unparsed tokens.
    pub fn is_empty(&self) -> bool {
        matches!(self.peeked, TokenKind::EoF)
    }

    /// Entry point for parsing any expression.
    pub fn parse_expr(&mut self) -> Option<ast::Term> {
        self.parse_expr_type()
    }

    /// Parses type annotations.
    pub fn parse_expr_type(&mut self) -> Option<ast::Term> {
        let mut expr = self.parse_expr_infix()?;
        while self.sat(|x| matches!(x, TokenKind::Colon)) {
            self.advance();
            let value = Box::new(expr);
            let ty = Box::new(self.parse_expr_infix()?);
            let span = Span::new(value.span.begin, ty.span.end);
            let kind = ast::TermKind::TypeAnno { value, ty };
            expr = ast::Term { span, kind };
        }
        Some(expr)
    }

    /// Parses custom infix operators.
    pub fn parse_expr_infix(&mut self) -> Option<ast::Term> {
        let mut expr = self.parse_expr_addition()?;
        while self.sat(TokenKind::is_identifier) {
            let kind = ast::BinaryOpKind::Custom(
                    Box::new(self.parse_expr_terminal()?));
            let left = Box::new(expr);
            let right = Box::new(self.parse_expr_addition()?);
            let span = Span::new(left.span.begin, right.span.end);
            let kind = ast::TermKind::BinaryOp { kind, left, right };
            expr = ast::Term { span, kind };
        }
        Some(expr)
    }

    /// Parses `+` and `-` binary operators.
    pub fn parse_expr_addition(&mut self) -> Option<ast::Term> {
        let mut expr = self.parse_expr_unary_prefix()?;
        while self.sat(|x| matches!(x, TokenKind::Plus | TokenKind::Minus)) {
            let kind = match self.advance() {
                TokenKind::Plus => ast::BinaryOpKind::Add,
                TokenKind::Minus => ast::BinaryOpKind::Subtract,
                _ => self.report(CompilerError::bug()
                        .reason("invalid addition operator"))?
            };
            let left = Box::new(expr);
            let right = Box::new(self.parse_expr_unary_prefix()?);
            let span = Span::new(left.span.begin, right.span.end);
            let kind = ast::TermKind::BinaryOp { kind, left, right };
            expr = ast::Term { span, kind };
        }
        Some(expr)
    }

    /// Parses the unary operator `-`.
    pub fn parse_expr_unary_prefix(&mut self) -> Option<ast::Term> {
        if self.sat(|x| matches!(x, TokenKind::Minus)) {
            let kind = match self.advance() {
                TokenKind::Minus => ast::UnaryOpKind::Negate,
                _ => self.report(CompilerError::bug()
                        .reason("invalid unary operator"))?
            };
            let mut span = self.span().clone();
            let value = Box::new(self.parse_expr_unary_prefix()?);
            span.end = value.span.end;
            let kind = ast::TermKind::UnaryOp { kind, value };
            Some(ast::Term { span, kind })
        } else {
            self.parse_expr_terminal()
        }
    }

    /// Parses literals and identifiers.
    pub fn parse_expr_terminal(&mut self) -> Option<ast::Term> {
        if self.sat(TokenKind::is_terminal) {
            let kind = match self.advance() {
                TokenKind::RawIdentifier { closed : false } => {
                    self.report(CompilerError::new()
                            .reason("raw identifier is missing a closing accent")
                            .note("consider adding a closing accent (`)"))?
                },
                x if x.is_identifier() => ast::TermKind::Variable,
                TokenKind::Integral => ast::TermKind::Integral,
                TokenKind::Primitive => ast::TermKind::Primitive,
                _ => self.report(CompilerError::bug()
                        .reason("invalid terminal kind"))?
            };
            let span = self.span().clone();
            Some(ast::Term { span, kind })
        } else {
            self.parse_expr_grouping()
        }
    }

    /// Parses groupings of expressions.
    pub fn parse_expr_grouping(&mut self) -> Option<ast::Term> {
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
                    .note("consider removing this token"))
        }
    }
}

impl<'a> Into<Lexer<'a>> for Parser<'a> {
    fn into(self) -> Lexer<'a> {
        self.lexer
    }
}

/// Generates the AST of this source code and reports any errors to this `IssueTracker`.
pub fn build_ast(src : &str, issues : &mut IssueTracker) -> Option<ast::Term> {
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
