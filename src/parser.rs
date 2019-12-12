use super::source_pos::Span;
use super::error::Error;
use super::scanner::Lexer;
use super::syntax::{
    token::*,
    ast::*
};

/// Takes a lexer and uses it to construct a parse tree.
pub struct Parser<'a> {
    lexer : Lexer<'a>,
    peek : Option<Result<Token<'a>, Error<'a>>>
}
impl<'a> Parser<'a> {
    /// Creates a new parser from this scanner.
    pub fn from(lexer : Lexer<'a>) -> Self {
        Parser { lexer, peek : None }
    }

    /// Consumes the parser and produces an abstract syntax tree.
    pub fn parse(mut self) -> Result<Expr<'a>, Error<'a>> {
        unimplemented!()
    }



    /// Advances the parser.
    fn advance(&mut self) -> Result<Option<Token<'a>>, Error<'a>> {
        let value = if self.peek.is_some() {
            self.peek.take()
        } else {
            self.lexer.next()
        };
        match value {
            Some(Ok(token)) => Ok(Some(token)),
            Some(Err(err)) => Err(err),
            None => Ok(None)
        }
    }
}