#![allow(dead_code)]

use super::Error;
use super::Result;
use super::super::collections::{
    token::{ Token, TokenType },
    parse_tree::*
};

use std::iter::Peekable;

macro_rules! matches {
    ($value:expr, $($pattern:tt)*) => ({
        match $value {
            $($pattern)* => true,
            _ => false
        }
    });
}

/// A struct which encapsulates the state of the parser.
pub struct Parser<'a, I> where
        I : Iterator<Item = Result<Token<'a>>> {
    scanner : Peekable<I>,
    row : usize,
    column : usize
}
impl<'a, I> Parser<'a, I> where
        I : Iterator<Item = Result<Token<'a>>> {
    /// Create a new parser using this scanner.
    pub fn from(scanner : I) -> Parser<'a, I> {
        Parser {
            scanner : scanner.peekable(),
            row : 0,
            column : 0
        }
    }

    /// Consumes the parser and produces an abstract syntax tree.
    pub fn parse(mut self) -> Result<Expr<'a>> {
        self.parse_expr()
    }

    /// Parses an expression.
    fn parse_expr(&mut self) -> Result<Expr<'a>> {
        self.parse_expr_frontier()
    }

    /// Parses expression literals and groupings.
    fn parse_expr_frontier(&mut self) -> Result<Expr<'a>> {
        match if let Some(value) = self.consume_if(|x| matches!(x,
                TokenType::String(..) |
                TokenType::Integer(..) |
                TokenType::Identifier(..)))? {
            Ok(Expr::Terminal { value })
        } else if let Some(_) = self.consume_if(|x| matches!(x, TokenType::LeftParen))? {
            let expr : Expr = self.parse_expr()?;
            if let Some(_) = self.consume_if(|x| matches!(x, TokenType::RightParen))? {
                Ok(expr)
            } else {
                Err("Expected ending ')' after expression")
            }
        } else {
            Err("Malformed expression")
        } {
            Ok(x) => Ok(x),
            Err(description) => Err(Error::Only {
                description,
                row : self.row,
                column : self.column
            })
        }
    }

    /// Consumes the next token only if the predicate is satisfied.
    fn consume_if(&mut self, f : impl Fn(&TokenType<'a>) -> bool) -> Result<Option<Token<'a>>> {
        if match self.scanner.peek() {
            Some(Ok(token)) => f(&token.flavour),
            Some(Err(_)) => true,
            None => false
        } {
            self.consume()
        } else {
            Ok(None)
        }
    }

    /// Consumes the next token.
    fn consume(&mut self) -> Result<Option<Token<'a>>> {
        match self.scanner.next() {
            Some(Ok(token)) => Ok(Some(token)),
            Some(Err(e)) => Err(e),
            None => Ok(None)
        }
    }
}

/// A trait which can be implemented by structs to offer a
/// way of converting into an abstract syntax tree.
pub trait Builder<'a> {
    /// Constructs a new scanner.
    fn into_ast(self) -> Result<Expr<'a>>;
}
impl<'a, I> Builder<'a> for I where
        I : Iterator<Item = Result<Token<'a>>> {
    fn into_ast(self) -> Result<Expr<'a>> {
        Parser::from(self).parse()
    }
}

fn substring<'a>(x : &'a str, start : usize, n : usize) -> &'a str {
    let end : usize = if let Some((i, _)) = x
            .char_indices()
            .take(n + start)
            .next() {
        i
    } else {
        x.len()
    };
    &x[start..end]
}