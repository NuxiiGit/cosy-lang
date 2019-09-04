#![allow(dead_code)]

use super::token::*;
use super::syntax_tree::*;

/// Parses an array of `Token`s into a parse tree.
pub fn parser<'a>(tokens : &[Token<'a>]) -> Option<Expr<'a>> {
    let mut tokens : Tokens = tokens
            .iter()
            .peekable();
    expression(&mut tokens)
}

/// Parse an expression.
fn expression<'a>(tokens : &mut Tokens) -> Option<Expr<'a>> {
    addition(tokens)
}

/// Parse a string of `+` and `-` operators.
fn addition<'a>(tokens : &mut Tokens) -> Option<Expr<'a>> {
    None
}

/// A type which represents the char iterator used by the lexer.
type Tokens<'a> = std::iter::Peekable<std::slice::Iter<'a, Token<'a>>>;