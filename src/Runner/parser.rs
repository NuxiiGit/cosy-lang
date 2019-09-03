#![allow(dead_code)]

use super::token::*;
use super::syntax_tree::*;

/// Parses an array of `Token`s into a parse tree.
pub fn parser(tokens : &[Token]) -> Result<Expr, &'static str> {
    let mut tokens : Tokens = tokens
            .iter()
            .peekable();
    expression(&mut tokens)
}

/// Parse an expression.
fn expression(tokens : &mut Tokens) -> Result<Expr, &'static str> {
    addition(tokens)
}

/// Parse a string of `+` and `-` operators.
fn addition(tokens : &mut Tokens) -> Result<Expr, &'static str> {
    
}

/// A type which represents the char iterator used by the lexer.
type Tokens<'a> = std::iter::Peekable<std::slice::Iter<Token>>;