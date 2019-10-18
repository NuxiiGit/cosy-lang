#![allow(dead_code)]

use super::Error;
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
        I : Iterator<Item = Result<Token<'a>, Error>> {
    scanner : Peekable<I>,
    row : usize,
    column : usize
}
impl<'a, I> Parser<'a, I> where
        I : Iterator<Item = Result<Token<'a>, Error>> {
    /// Create a new parser using this scanner.
    pub fn from(scanner : I) -> Parser<'a, I> {
        Parser {
            scanner : scanner.peekable(),
            row : 0,
            column : 0
        }
    }

    /// Consumes the parser and produces an abstract syntax tree.
    pub fn parse(mut self) -> Result<Expr<'a>, Error> {
        self.parse_expr()
    }

    /// Parses an expression.
    fn parse_expr(&mut self) -> Result<Expr<'a>, Error> {
        let mut left : Expr = self.parse_expr_equality()?;
        while let Some(ident) = self.consume_if(|x| matches!(x, TokenType::Operator(..)))? {
            let right : Expr = self.parse_expr_equality()?;
            left = Expr::Call {
                ident,
                args : vec![left, right]
            }
        }
        Ok(left)
    }

    /// Parses a stream of `!` and `=` binary operators.
    fn parse_expr_equality(&mut self) -> Result<Expr<'a>, Error> {
        let mut left : Expr = self.parse_expr_inequality()?;
        while let Some(ident) = self.consume_if(|x| matches!(x, TokenType::Operator(op) if
                matches!(substr(op, 0, 1), "!" | "=")))? {
            let right : Expr = self.parse_expr_inequality()?;
            left = Expr::Call {
                ident,
                args : vec![left, right]
            }
        }
        Ok(left)
    }

    /// Parses a stream of `<` and `>` binary operators.
    fn parse_expr_inequality(&mut self) -> Result<Expr<'a>, Error> {
        let mut left : Expr = self.parse_expr_disjunction()?;
        while let Some(ident) = self.consume_if(|x| matches!(x, TokenType::Operator(op) if
                matches!(substr(op, 0, 1), "<" | ">")))? {
            let right : Expr = self.parse_expr_disjunction()?;
            left = Expr::Call {
                ident,
                args : vec![left, right]
            }
        }
        Ok(left)
    }

    /// Parses a stream of `|` binary operators.
    fn parse_expr_disjunction(&mut self) -> Result<Expr<'a>, Error> {
        let mut left : Expr = self.parse_expr_conjunction()?;
        while let Some(ident) = self.consume_if(|x| matches!(x, TokenType::Operator(op) if
                matches!(substr(op, 0, 1), "|")))? {
            let right : Expr = self.parse_expr_conjunction()?;
            left = Expr::Call {
                ident,
                args : vec![left, right]
            }
        }
        Ok(left)
    }

    /// Parses a stream of `&` binary operators.
    fn parse_expr_conjunction(&mut self) -> Result<Expr<'a>, Error> {
        let mut left : Expr = self.parse_expr_addition()?;
        while let Some(ident) = self.consume_if(|x| matches!(x, TokenType::Operator(op) if
                matches!(substr(op, 0, 1), "&")))? {
            let right : Expr = self.parse_expr_addition()?;
            left = Expr::Call {
                ident,
                args : vec![left, right]
            }
        }
        Ok(left)
    }

    /// Parses a stream of `+` and `-` binary operators.
    fn parse_expr_addition(&mut self) -> Result<Expr<'a>, Error> {
        let mut left : Expr = self.parse_expr_multiplication()?;
        while let Some(ident) = self.consume_if(|x| matches!(x, TokenType::Operator(op) if
                matches!(substr(op, 0, 1), "+" | "-")))? {
            let right : Expr = self.parse_expr_multiplication()?;
            left = Expr::Call {
                ident,
                args : vec![left, right]
            }
        }
        Ok(left)
    }

    /// Parses a stream of `*`, `/`, and `%` binary operators.
    fn parse_expr_multiplication(&mut self) -> Result<Expr<'a>, Error> {
        let mut left : Expr = self.parse_expr_unary()?;
        while let Some(ident) = self.consume_if(|x| matches!(x, TokenType::Operator(op) if
                matches!(substr(op, 0, 1), "*" | "/" | "%")))? {
            let right : Expr = self.parse_expr_unary()?;
            left = Expr::Call {
                ident,
                args : vec![left, right]
            }
        }
        Ok(left)
    }

    /// Parses a stream of prefix unary operators.
    fn parse_expr_unary(&mut self) -> Result<Expr<'a>, Error> {
        if let Some(ident) = self.consume_if(|x| matches!(x, TokenType::Operator(..)))? {
            let right : Expr = self.parse_expr_unary()?;
            Ok(Expr::Call {
                ident,
                args : vec![right]
            })
        } else {
            self.parse_expr_member()
        }
    }

    /// Parses a stream of member accesses.
    fn parse_expr_member(&mut self) -> Result<Expr<'a>, Error> {
        let mut expr : Expr = self.parse_expr_frontier()?;
        while let Some(ident) = self.consume_if(|x| matches!(x, TokenType::Identifier(..)))? {
            expr = Expr::Member {
                ident,
                expr : Box::new(expr)
            }
        }
        Ok(expr)
    }

    /// Parses expression literals and groupings.
    fn parse_expr_frontier(&mut self) -> Result<Expr<'a>, Error> {
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
            Err(description) => Err(Error {
                description,
                row : self.row,
                column : self.column
            })
        }
    }

    /// Consumes the next token only if the predicate is satisfied.
    fn consume_if(&mut self, f : impl Fn(&TokenType<'a>) -> bool) -> Result<Option<Token<'a>>, Error> {
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
    fn consume(&mut self) -> Result<Option<Token<'a>>, Error> {
        match self.scanner.next() {
            Some(Ok(token)) => {
                self.row = token.row;
                self.column = token.column;
                Ok(Some(token))
            },
            Some(Err(e)) => Err(e),
            None => Ok(None)
        }
    }
}

/// A trait which can be implemented by structs to offer a
/// way of converting into an abstract syntax tree.
pub trait Builder<'a> {
    /// Constructs a new scanner.
    fn into_ast(self) -> Result<Expr<'a>, Error>;
}
impl<'a, I> Builder<'a> for I where
        I : Iterator<Item = Result<Token<'a>, Error>> {
    fn into_ast(self) -> Result<Expr<'a>, Error> {
        Parser::from(self).parse()
    }
}

/// Returns a substring of this `str`.
fn substr<'a>(x : &'a str, start : usize, n : usize) -> &'a str {
    let end : usize = if let Some((i, _)) = x
            .char_indices()
            .skip(n + start)
            .next() {
        i
    } else {
        x.len()
    };
    &x[start..end]
}