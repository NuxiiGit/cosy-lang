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
        if matches!(self.token(), TokenKind::LeftBrace) {
            self.parse_expr_block()
        } else {
            self.parse_expr_terminal()
        }
    }

    /// Parses block expressions.
    pub fn parse_expr_block(&mut self) -> Expr {
        let mut span = self.span().clone();
        let lbrace = self.advance_if(|x| matches!(x, TokenKind::LeftBrace)).is_some();
        let mut body : Vec<Stmt> = Vec::new();
        let ret = if !matches!(self.token(), TokenKind::RightBrace) {
            loop {
                let mut span = self.span().clone();
                let kind = if matches!(self.token(), TokenKind::SemiColon | TokenKind::RightBrace) {
                    StmtKind::NoOp
                } else {
                    let inner = Box::new(self.parse_expr());
                    span.end = inner.span.end;
                    StmtKind::Expr { inner }
                };
                if matches!(self.token(), TokenKind::SemiColon) {
                    span.end = self.span().end;
                    self.advance();
                } else if let StmtKind::Expr { inner } = kind {
                    // exit with expression
                    break Some(*inner);
                } else {
                    // exit with none
                    break None;
                }
                body.push(Stmt { span, kind });
            }
        } else {
            None
        };
        let rbrace = matches!(self.token(), TokenKind::RightBrace);
        if rbrace {
            // if the block can be closed correctly
            // then consume the closing brace
            span.end = self.span().end;
            self.advance();
        } else if let Some(stmt) = body.last() {
            span.end = stmt.span.end;
        };
        let kind = ExprKind::Block { lbrace, rbrace, body };
        Expr { span, kind }
    }

    /// Parses literals and identifiers.
    pub fn parse_expr_terminal(&mut self) -> Expr {
        let span = self.span().clone();
        let kind = if self.advance_if(TokenKind::is_graphic).is_some() {
            ExprKind::Variable
        } else if self.advance_if(TokenKind::is_integral).is_some() {
            ExprKind::Integral
        } else if matches!(self.token(), TokenKind::LeftParen) {
            return self.parse_expr_grouping();
        } else {
            ExprKind::Malformed
        };
        Expr { span, kind }
    }

    /// Parses groupings of expressions.
    pub fn parse_expr_grouping(&mut self) -> Expr {
        let mut span = self.span().clone();
        let lparen = self.advance_if(|x| matches!(x, TokenKind::LeftParen)).is_some();
        let inner = if matches!(self.token(), TokenKind::RightParen) {
            // empty expression
            None
        } else {
            // parse groupings
            Some(Box::new(self.parse_expr()))
        };
        let rparen = matches!(self.token(), TokenKind::RightParen);
        if rparen {
            // if the grouping can be closed correctly
            // then consume the closing paren
            span.end = self.span().end;
            self.advance();
        } else if let Some(expr) = &inner {
            span.end = expr.span.end;
        }
        let kind = ExprKind::Grouping { lparen, rparen, inner };
        Expr { span, kind }
    }
}
impl<'a> From<&'a str> for Parser<'a> {
    fn from(src : &'a str) -> Self {
        let mut lexer = Lexer::from(src);
        let peeked = lexer.generate_token();
        Self { lexer, peeked }
    }
}
