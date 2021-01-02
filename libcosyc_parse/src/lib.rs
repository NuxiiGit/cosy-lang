pub mod syntax;

use libcosyc_diagnostic::{
    source::Span,
    error::{ IssueTracker, CompilerError, ErrorLevel }
};
use libcosyc_scan::{ Lexer, token::TokenKind };
use crate::syntax as ast;
use std::mem;

fn generate_token(lexer : &mut Lexer) -> TokenKind {
    loop {
        // ignore comment tokens
        let token = lexer.generate_token();
        if !matches!(token, TokenKind::Comment) {
            break token;
        }
    }
}

/// Produces a concrete syntax tree from concrete syntax.
pub struct Parser<'a> {
    lexer : Lexer<'a>,
    peeked : TokenKind,
    span_previous : Span
}

impl Parser<'_> {
    /// Returns the span of the current lexeme.
    pub fn span(&self) -> &Span {
        self.lexer.span()
    }

    /// Returns the span of the previous lexeme.
    pub fn span_previous(&self) -> &Span {
        &self.span_previous
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
}

impl<'a> From<Lexer<'a>> for Parser<'a> {
    fn from(mut lexer : Lexer<'a>) -> Self {
        let peeked = generate_token(&mut lexer);
        let span_previous = Span::default();
        Self { lexer, peeked, span_previous }
    }
}
