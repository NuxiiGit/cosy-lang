pub mod lex;
pub mod syntax;

use lex::{ Lexer, lexeme::* };
use syntax::*;

use libcosyc_source::Span;

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
    pub fn matches(&mut self, p : fn(&TokenKind) -> bool) -> bool {
        p(self.token())
    }

    /// Parses literals, identifiers, and groupings of expressions.
    pub fn parse_expr_terminal(&mut self) -> Expr {
        if self.matches(TokenKind::is_terminal) {
            let span = self.span().clone();
            let kind = match self.advance() {
                TokenKind::Identifier(IdentifierKind::Graphic) => Some(ExprKind::Variable),
                TokenKind::Literal(literal) => Some(ExprKind::Value(match literal {
                    LiteralKind::Integral => ValueKind::Integral
                })),
                _ => None
            };
            Expr { span, kind  }
        } else {
            self.parse_expr_groupings()
        }
    }

    /// Parses groupings of expressions.
    pub fn parse_expr_groupings(&mut self) -> Expr {
        unimplemented!()
    }
}
impl<'a> From<&'a str> for Parser<'a> {
    fn from(src : &'a str) -> Self {
        let mut lexer = Lexer::from(src);
        let peeked = lexer.generate_token();
        Self { lexer, peeked }
    }
}

