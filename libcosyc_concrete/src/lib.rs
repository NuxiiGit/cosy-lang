pub mod lex;

use lex::{ Lexer, TokenKind, LiteralKind, IdentifierKind };

use libcosyc_source::Span;

use std::mem;

/// Represents the different primitive variants.
#[derive(Debug)]
pub enum ValueKind {
    Integral
}

/// Represents a kind of terminal value.
#[derive(Debug)]
pub enum TerminalKind {
    Variable,
    Value(ValueKind)
}

/// Represents a kind of expression.
#[derive(Debug)]
pub enum ExprKind {
    Terminal(TerminalKind)
}

/// Represents expression information
#[derive(Debug)]
pub struct Expr {
    span : Span,
    kind : Option<ExprKind>
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
    pub fn matches(&mut self, p : fn(&TokenKind) -> bool) -> Option<TokenKind> {
        if p(self.token()) {
            Some(self.advance())
        } else {
            None
        }
    }

    /// Parses literals, identifiers, and groupings of expressions.
    pub fn parse_expr_terminal(&mut self) -> Expr {
        let terminal_kind = self.matches(TokenKind::is_terminal).and_then(|x| match x {
            TokenKind::Identifier(IdentifierKind::Graphic) => Some(TerminalKind::Variable),
            _ => None
        });
        let kind = terminal_kind.map(ExprKind::Terminal);
        let span = self.span().clone();
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

