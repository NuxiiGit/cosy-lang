pub mod lex;

use lex::{ Lexer, TokenKind, LiteralKind, IdentifierKind };

use libcosyc_diagnostics::{ Diagnostic, Session, IssueTracker, span::Span };

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

