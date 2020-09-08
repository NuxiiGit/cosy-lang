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
        let mut span = self.span().clone();
        let kind = match self.advance() {
            TokenKind::Identifier(IdentifierKind::Graphic) => ExprKind::Variable,
            TokenKind::Literal(LiteralKind::Integral) => ExprKind::Integral,
            x => {
                // implement groupings
                unimplemented!()
            }
        };
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

