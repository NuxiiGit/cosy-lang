pub mod lex;

use lex::{ Lexer, TokenKind, LiteralKind, IdentifierKind };

use libcosyc_diagnostics::{ Diagnostic, Session, IssueTracker, span::Span };

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
    issues : &'a mut IssueTracker,
    lexer : Lexer<'a>,
    peeked : TokenKind
}
impl<'a> From<&'a mut Session> for Parser<'a> {
    fn from(sess : &'a mut Session) -> Self {
        let issues = &mut sess.issues;
        let mut lexer = Lexer::from(&sess.src as &str);
        let peeked = lexer.generate_token();
        Self { issues, lexer, peeked }
    }
}

