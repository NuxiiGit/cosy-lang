pub mod lex;

use lex::{ Lexer, TokenKind, LiteralKind, IdentifierKind };

use libcosyc_diagnostics::{ Diagnostic, Session, span::Span };

/// Produces a concrete syntax tree from concrete syntax.
pub struct Parser<'a> {
    sess : &'a mut Session,
    lexer : Lexer<'a>,
    peeked : TokenKind
}
impl<'a> From<&'a mut Session> for Parser<'a> {
        fn from(sess : &'a mut Session) -> Self {
                let mut lexer = Lexer::from(&sess.src as &str);
                let peeked = lexer.generate_token();
                Self { sess, lexer, peeked }
        }
}

