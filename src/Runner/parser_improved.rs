#![allow(dead_code)]

use super::Result;
use super::collections::{
    token::*,
    syntax_tree::*
};

use std::iter::Peekable;
use std::str::CharIndices;
use std::error;
use std::fmt;

/// A macro for matching a value with a pattern.
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
    /// Constructs a new parser.
    pub fn new(scanner : I) -> Parser<'a, I> {
        Parser {
            scanner : scanner.peekable(),
            row : 0,
            column : 0
        }
    }
    
    /// Constumes the parser and produces a syntax tree of its expression.
    pub fn parse(mut self) -> std::result::Result<Expr<'a>, Vec<super::RunnerError>> {
        match self.parse_expr() {
            Ok(expr) => Ok(expr),
            Err(err) => Err(vec![err])
        }
    }
    
    /// Parses an expression.
    fn parse_expr(&mut self) -> Result<Expr<'a>> {
        let mut left : Expr = self.parse_expr_equality()?;
        while let Some(operator) = self.consume_if(|x| matches!(x, TokenType::Operator(..))) {
            let right : Expr = self.parse_expr_equality()?;
            left = Expr::Binary {
                operator : operator,
                left : Box::new(left),
                right : Box::new(right)
            };
        }
        Ok(left)
    }

    /// Parses a string of `!=` and `==` binary operators.
    fn parse_expr_equality(&mut self) -> Result<Expr<'a>> {
        let mut left : Expr = self.parse_expr_inequality()?;
        while let Some(operator) = self.consume_if(|x| matches!(x, TokenType::Operator(op) if
                matches!(substring(op, 0, 1), "!" | "="))) {
            let right : Expr = self.parse_expr_inequality()?;
            left = Expr::Binary {
                operator : operator,
                left : Box::new(left),
                right : Box::new(right)
            };
        }
        Ok(left)
    }

    /// Parses a string of `!=` and `==` binary operators.
    fn parse_expr_inequality(&mut self) -> Result<Expr<'a>> {
        let mut left : Expr = self.parse_expr_addition()?;
        while let Some(operator) = self.consume_if(|x| matches!(x, TokenType::Operator(op) if
                matches!(substring(op, 0, 1), ">" | "<"))) {
            let right : Expr = self.parse_expr_addition()?;
            left = Expr::Binary {
                operator : operator,
                left : Box::new(left),
                right : Box::new(right)
            };
        }
        Ok(left)
    }

    /// Parses a string of `+` and `-` binary operators.
    fn parse_expr_addition(&mut self) -> Result<Expr<'a>> {
        let mut left : Expr = self.parse_expr_multiplication()?;
        while let Some(operator) = self.consume_if(|x| matches!(x, TokenType::Operator(op) if
                matches!(substring(op, 0, 1), "+" | "-"))) {
            let right : Expr = self.parse_expr_multiplication()?;
            left = Expr::Binary {
                operator : operator,
                left : Box::new(left),
                right : Box::new(right)
            };
        }
        Ok(left)
    }

    /// Parses a string of `*`, `/`, and '%' binary operators.
    fn parse_expr_multiplication(&mut self) -> Result<Expr<'a>> {
        let mut left : Expr = self.parse_expr_unary()?;
        while let Some(operator) = self.consume_if(|x| matches!(x, TokenType::Operator(op) if
                matches!(substring(op, 0, 1), "*" | "/" | "%"))) {
            let right : Expr = self.parse_expr_unary()?;
            left = Expr::Binary {
                operator : operator,
                left : Box::new(left),
                right : Box::new(right)
            };
        }
        Ok(left)
    }

    /// Parses any sort of chained unary operators.
    fn parse_expr_unary(&mut self) -> Result<Expr<'a>> {
        if let Some(operator) = self.consume_if(|x| matches!(x, TokenType::Operator(..))) {
            let right : Expr = self.parse_expr_unary()?;
            Ok(Expr::Unary {
                operator : operator,
                right : Box::new(right)
            })
        } else {
            self.parse_expr_member()
        }
    }

    /// Parses a chain of identifiers.
    fn parse_expr_member(&mut self) -> Result<Expr<'a>> {
        let mut expr : Expr = self.parse_expr_frontier()?;
        while let Some(ident) = self.consume_if(|x| matches!(x, TokenType::Identifier(..))) {
            expr = Expr::Member {
                left : Box::new(expr),
                field : ident
            }
        }
        Ok(expr)
    }

    /// Parses the frontier of an expression.
    fn parse_expr_frontier(&mut self) -> Result<Expr<'a>> {
        if let Some(literal) = self.consume_if(|x| matches!(x,
                TokenType::String(..) |
                TokenType::Integer(..))) {
            Ok(Expr::Literal {
                value : literal
            })
        } else if let Some(_) = self.consume_if(|x| matches!(x, TokenType::LeftParen)) {
            let expr : Expr = self.parse_expr()?;
            if let Some(_) = self.consume_if(|x| matches!(x, TokenType::RightParen)) {
                Ok(expr)
            } else {
                Err(Box::new(ParserError {
                    description : "Expected ending ')' after expression",
                    row : self.row,
                    column : self.column
                }))
            }
        } else {
            Err(Box::new(ParserError {
                description : "Malformed expression",
                row : self.row,
                column : self.column
            }))
        }
    }

    /// Consumes the next token if the closure returns true.
    fn consume_if(&mut self, f : impl Fn(&TokenType<'a>) -> bool) -> Option<Token<'a>> {
        match self.scanner.peek() {
            Some(Ok(token)) => {
                if f(&token.flavour) {
                    self.consume_next()
                } else {
                    None
                }
            },
            _ => None
        }
    }

    /// Consumes the next token.
    fn consume_next(&mut self) -> Option<Result<Token<'a>>> {
        let result : Option<Result<Token>> = self.scanner.next();
        if let Some(Ok(token)) = result {
            self.row = token.row;
            self.column = token.column;
        }
        result
    }

    /// Throw a parser error.
    fn make_error(&mut self, description : &'static str) -> Result<()> {
        Err(Box::new(ParserError {
            description,
            row : self.row,
            column : self.column
        }))
    }
}

/// An error type which represents a parser error.
#[derive(Debug)]
pub struct ParserError {
    pub description : &'static str,
    pub row : usize,
    pub column : usize
}
impl fmt::Display for ParserError {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error at (row. {}, col. {}): {}",
                self.row, self.column, self.description)
    }
}
impl error::Error for ParserError {}

/// Returns a substring of this `str`.
fn substring<'a>(s : &'a str, i : usize, n : usize) -> &'a str {
    let start : usize = i;
    let end : usize = if let Some((x, _)) = s
            .char_indices()
            .take(n + start)
            .next() {
        x
    } else {
        s.len()
    };
    &s[start..end]
}