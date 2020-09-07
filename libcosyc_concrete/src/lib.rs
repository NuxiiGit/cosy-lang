pub mod lex;

use lex::{ Lexer, TokenKind, LiteralKind, IdentifierKind };

use libcosyc_source::Span;

use std::mem;

/// Represents the different primitive variants.
#[derive(Debug)]
pub enum ValueKind {
    Integral
}

/// Represents a kind of terminal expression.
#[derive(Debug)]
pub enum TerminalKind {
    Variable,
    Value(ValueKind)
}

/// Represents a terminal value
#[derive(Debug)]
pub struct Terminal {
    span : Span,
    kind : TerminalKind
}

/// Represents a kind of expression.
#[derive(Debug)]
pub enum ExprKind {
    Terminal(Terminal)
}

/// Represents expression information
#[derive(Debug)]
pub struct Expr {
    kind : ExprKind
}

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

    /// Returns the current location of the parser.
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
}
impl<'a> From<&'a str> for Parser<'a> {
    fn from(src : &'a str) -> Self {
        let mut lexer = Lexer::from(src);
        let peeked = lexer.generate_token();
        Self { lexer, peeked }
    }
}

