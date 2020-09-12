use crate::lex::{ Lexer, lexeme::* };
use crate::syntax::*;

use libcosyc_diagnostics::source::Span;

use std::mem;

/// Produces a concrete syntax tree from concrete syntax.
pub struct Parser<'a> {
    lexer : Lexer<'a>,
    peeked : TokenKind
}
impl<'a> Parser<'a> {
    /// Returns a reference to the current token kind.
    pub fn token(&self) -> &TokenKind {
        &self.peeked
    }

    /// Returns a the current lexeme span.
    pub fn span(&self) -> &Span {
        self.lexer.span()
    }

    /// Advances the parser and returns the the previous lexeme.
    pub fn advance(&mut self) -> TokenKind {
        let next = self.lexer.generate_token();
        mem::replace(&mut self.peeked, next)
    }

    /// Returns whether the current peeked token holds a predicate.
    pub fn advance_if(&mut self, p : fn(&TokenKind) -> bool) -> Option<TokenKind> {
        if p(self.token()) {
            Some(self.advance())
        } else {
            None
        }
    }

    /// Entry point for parsing expressions.
    pub fn parse_expr(&mut self) -> Expr {
        self.parse_expr_stmt()
    }

    /// Entry point for parsing statement expressions.
    pub fn parse_expr_stmt(&mut self) -> Expr {
        if self.advance_if(|x| matches!(x, TokenKind::LeftBrace)).is_some() {
            unimplemented!()
        } else {
            self.parse_expr_terminal()
        }
    }

    /// Parses literals, identifiers, and groupings of expressions.
    pub fn parse_expr_terminal(&mut self) -> Expr {
        let mut span = self.span().clone();
        let kind = if self.advance_if(TokenKind::is_graphic).is_some() {
            ExprKind::Variable
        } else if self.advance_if(TokenKind::is_integral).is_some() {
            ExprKind::Integral
        } else if self.advance_if(|x| matches!(x, TokenKind::LeftParen)).is_some() {
            if matches!(self.token(), TokenKind::RightParen) {
                // empty expression
                span.end = self.span().end;
                self.advance();
                ExprKind::Empty
            } else {
                // parse groupings
                let inner = Box::new(self.parse_expr());
                let unclosed = !matches!(self.token(), TokenKind::RightParen);
                if unclosed {
                    span.end = inner.span.end;
                } else {
                    // if the grouping can be closed correctly
                    // then consume the closing paren
                    span.end = self.span().end;
                    self.advance();
                }
                ExprKind::Grouping { unclosed, inner }
            }
        } else {
            ExprKind::Malformed
        };
        Expr { span, kind }
    }

    /*
    /// Parses statements.
    pub fn parse_stmt(&mut self) -> Stmt {
        let mut span = self.span().clone();
        let kind = if self.advance_if(|x| !matches!(x, TokenKind::SemiColon)).is_some() {
            let inner = Box::new(self.parse_expr());
            span.end = inner.span.end;
            StmtKind::Expr { inner }
        } else {
            StmtKind::NoOp
        };
        Stmt { span, kind }
    }
    */
}
impl<'a> From<&'a str> for Parser<'a> {
    fn from(src : &'a str) -> Self {
        let mut lexer = Lexer::from(src);
        let peeked = lexer.generate_token();
        Self { lexer, peeked }
    }
}
