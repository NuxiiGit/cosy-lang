#![allow(dead_code)]

use super::lexer::Lexer;
use super::error::Error;
use super::token::*;
use super::syntax_tree::*;

/// A struct which encapsulates the state of the parser.
pub struct Parser<'a> {
    lexer : Lexer<'a>
}
impl<'a> Parser<'a> {
    /// Parses an expression and returns its syntax tree.
    /// # Errors
    /// Errors are logged to `error::Error`, and can be obtained using:
    /// ```
    /// let errors = error::Error::log();
    /// ```
    pub fn parse(context : &'a str) -> Option<SyntaxTree<'a>> {
        let mut parser : Parser = Parser {
            lexer : Lexer::lex(context)
        };
        let expr : Expr = parser.parse_expression()?;
        Some(SyntaxTree::Expression(expr))
    }

    /// Parses an expression.
    fn parse_expression(&mut self) -> Option<Expr<'a>> {
        self.parse_addition()
    }
    
    /// Parses an string of `+` and `-` binary operators.
    fn parse_addition(&mut self) -> Option<Expr<'a>> {
        None
    }
}