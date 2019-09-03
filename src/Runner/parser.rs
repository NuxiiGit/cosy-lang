#![allow(dead_code)]

use super::token::*;
use super::syntax_tree::*;
use super::error::*;

/// Parses an array of `Token`s into a parse tree.
pub fn parser<'a>(tokens : &[Token<'a>]) -> Result<Expr<'a>, CompileError<'static>> {
    let mut tokens : Tokens = tokens
            .iter()
            .peekable();
    expression(&mut tokens)
}

/// Parse an expression.
fn expression<'a>(tokens : &mut Tokens) -> Result<Expr<'a>, CompileError<'static>> {
    addition(tokens)
}

/// Parse a string of `+` and `-` operators.
fn addition<'a>(tokens : &mut Tokens) -> Result<Expr<'a>, CompileError<'static>> {
    Err(CompileError::new("Not implemented", 0, 0))
}

/// A type which represents the char iterator used by the lexer.
type Tokens<'a> = std::iter::Peekable<std::slice::Iter<'a, Token<'a>>>;