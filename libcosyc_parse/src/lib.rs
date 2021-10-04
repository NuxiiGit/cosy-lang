pub mod syntax;

use libcosyc_diagnostic::{
    source::Span,
    error::{ CompilerError, IssueTracker }
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

/// The minimum operator precedence.
const MIN_OPERATOR_PRECEDENCE : u8 = 0;

/// The maximum operator precedence.
const MAX_OPERATOR_PRECEDENCE : u8 = 9;

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

    /// Returns the substring of the current lexeme.
    pub fn substring(&self) -> &'a str {
        self.lexer.substring()
    }

    /// Returns whether the current token satisfies a predicate `p`.
    /// The function will always return `false` for the EoF token.
    pub fn sat(&self, p : impl FnOnce(&TokenKind) -> bool) -> bool {
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

    /// Returns the token if it satisfies the predicate `p`, otherwise the error is reported.
    pub fn expect(&mut self, p : fn(&TokenKind) -> bool, error : CompilerError) -> Option<TokenKind> {
        if self.sat(p) {
            Some(self.advance())
        } else {
            self.issues.report_error(error)
        }
    }

    /// Returns whether the parser contains additional unparsed tokens.
    pub fn is_empty(&self) -> bool {
        matches!(self.peeked, TokenKind::EoF)
    }

    /// Entry point for parsing any expression.
    pub fn parse_expr(&mut self) -> Option<ast::Term> {
        self.parse_expr_annotation()
    }

    /// Parses type annotations.
    pub fn parse_expr_annotation(&mut self) -> Option<ast::Term> {
        let value = self.parse_expr_binary(MIN_OPERATOR_PRECEDENCE)?;
        if self.sat(|x| matches!(x, TokenKind::Colon)) {
            self.advance();
            let value = Box::new(value);
            let datatype = Box::new(self.parse_expr_terminal()?);
            let span = value.span.join(&datatype.span);
            let kind = ast::TermKind::TypeAnno { value, datatype };
            Some(ast::Term { span, kind })
        } else {
            Some(value)
        }
    }

    /// Parses a binary operator with this precedence.
    pub fn parse_expr_binary(&mut self, expected_precedence : u8) -> Option<ast::Term> {
        if expected_precedence > MAX_OPERATOR_PRECEDENCE {
            return self.parse_expr_unary();
        }
        let mut expr = self.parse_expr_binary(expected_precedence + 1)?;
        while self.sat(|x| matches!(x, TokenKind::Operator { precedence }
                if *precedence == expected_precedence )) {
            self.advance();
            let op = self.span().clone();
            let left = Box::new(expr);
            let right = Box::new(self.parse_expr_binary(expected_precedence + 1)?);
            let span = left.span.join(&right.span);
            let kind = ast::TermKind::BinaryOp { op, left, right };
            expr = ast::Term { span, kind };
        }
        Some(expr)
    }

    /// Parses unary operators.
    pub fn parse_expr_unary(&mut self) -> Option<ast::Term> {
        if self.sat(|x| matches!(x, TokenKind::Operator { .. })) {
            self.advance();
            let op = self.span().clone();
            let value = Box::new(self.parse_expr_terminal()?);
            let span = op.join(&value.span);
            let kind = ast::TermKind::UnaryOp { op, value };
            Some(ast::Term { span, kind })
        } else {
            self.parse_expr_terminal()
        }
    }

    /// Parses literals and identifiers.
    pub fn parse_expr_terminal(&mut self) -> Option<ast::Term> {
        if self.sat(TokenKind::is_identifier) {
            let kind = match self.advance() {
                TokenKind::RawIdentifier { closed : false } => {
                    self.issues.report_error(CompilerError::new()
                            .span(self.span())
                            .reason("raw identifier is missing a closing accent")
                            .note("consider adding a closing accent (`)"))?
                },
                _ => ast::TermKind::Variable
            };
            let span = self.span().clone();
            Some(ast::Term { span, kind })
        } else if self.sat(TokenKind::is_terminal) {
            let token = self.advance();
            let span = self.span().clone();
            let kind = match token {
                TokenKind::Integral => ast::TermKind::Integral { radix : 10 },
                _ => self.issues.report_error(CompilerError::bug()
                        .span(self.span())
                        .reason("invalid terminal kind"))?
            };
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
            self.issues.report_error(CompilerError::new()
                    .span(self.span())
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
                .reason("unparsed tokens at the end of this file"))
    }
}
