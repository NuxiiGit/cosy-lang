#![allow(dead_code)]

use super::lexer::Lexer;
use super::error::Error;
use super::token::*;
use super::syntax_tree::*;
use std::iter::Peekable;

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
pub struct Parser<'a> {
    lexer : Peekable<Lexer<'a>>,
    row : usize,
    column : usize
}
impl<'a> Parser<'a> {
    /// Constructs a new parser.
    /// # Errors
    /// Errors are logged to `error::Error`, and can be obtained using:
    /// ```
    /// let errors = error::Error::log();
    /// ```
    pub fn new(scanner : Lexer<'a>) -> Parser<'a> {
        Parser {
            lexer : scanner.peekable(),
            row : 0,
            column : 0
        }
    }

    /// Parses an expression and returns its syntax tree.
    pub fn into_ast(mut self) -> Option<SyntaxTree<'a>> {
        let expr : Expr = self.parse_expr()?;
        Some(SyntaxTree::Expression(expr))
    }

    /// Parses an expression.
    fn parse_expr(&mut self) -> Option<Expr<'a>> {
        let mut left : Expr = self.parse_expr_addition()?;
        while let Some(token) = self.consume_if(|x| matches!(x, TokenType::Operator(..))) {
            let right : Expr = self.parse_expr_addition()?;
            left = Expr::Binary(token, Box::new(left), Box::new(right));
        }
        Some(left)
    }

    /// Parses a string of `!=` and `==` binary operators.
    fn parse_expr_equality(&mut self) -> Option<Expr<'a>> {
        let mut left : Expr = self.parse_expr_inequality()?;
        while let Some(token) = self.consume_if(|x| matches!(x, TokenType::Operator(op) if
                matches!(op.substring(0, 1), "!" | "="))) {
            let right : Expr = self.parse_expr_inequality()?;
            left = Expr::Binary(token, Box::new(left), Box::new(right));
        }
        Some(left)
    }

    /// Parses a string of `!=` and `==` binary operators.
    fn parse_expr_inequality(&mut self) -> Option<Expr<'a>> {
        let mut left : Expr = self.parse_expr_addition()?;
        while let Some(token) = self.consume_if(|x| matches!(x, TokenType::Operator(op) if
                matches!(op.substring(0, 1), ">" | "<"))) {
            let right : Expr = self.parse_expr_addition()?;
            left = Expr::Binary(token, Box::new(left), Box::new(right));
        }
        Some(left)
    }

    /// Parses a string of `+` and `-` binary operators.
    fn parse_expr_addition(&mut self) -> Option<Expr<'a>> {
        let mut left : Expr = self.parse_expr_multiplication()?;
        while let Some(token) = self.consume_if(|x| matches!(x, TokenType::Operator(op) if
                matches!(op.substring(0, 1), "+" | "-"))) {
            let right : Expr = self.parse_expr_multiplication()?;
            left = Expr::Binary(token, Box::new(left), Box::new(right));
        }
        Some(left)
    }

    /// Parses a string of `*`, `/`, and '%' binary operators.
    fn parse_expr_multiplication(&mut self) -> Option<Expr<'a>> {
        let mut left : Expr = self.parse_expr_frontier()?;
        while let Some(token) = self.consume_if(|x| matches!(x, TokenType::Operator(op) if
                matches!(op.substring(0, 1), "*" | "/" | "%"))) {
            let right : Expr = self.parse_expr_frontier()?;
            left = Expr::Binary(token, Box::new(left), Box::new(right));
        }
        Some(left)
    }

    /// Parses the frontier of an expression.
    fn parse_expr_frontier(&mut self) -> Option<Expr<'a>> {
        if let Some(token) = self.consume_if(|x| matches!(x,
                TokenType::String(..) |
                TokenType::Integer(..))) {
            return Some(Expr::Literal(token));
        } else {
            if let Some(_) = self.consume_if(|x| matches!(x, TokenType::LeftParen)) {
                let expr : Expr = self.parse_expr()?;
                if let Some(_) = self.consume_if(|x| matches!(x, TokenType::RightParen)) {
                    return Some(expr);
                } else {
                    self.error("Expected ending ')' after expression");
                }
            } else {
                self.error("Malformed expression");
            }
        }
        None
    }

    /// Consumes the next token if the closure returns true.
    fn consume_if(&mut self, f : impl Fn(&TokenType<'a>) -> bool) -> Option<Token<'a>> {
        let token : &Token = self.lexer.peek()?;
        if f(&token.flavour) {
            self.consume_next()
        } else {
            None
        }
    }

    /// Consumes the next token.
    fn consume_next(&mut self) -> Option<Token<'a>> {
        if let Some(token) = self.lexer.next() {
            self.row = token.row;
            self.column = token.column;
            Some(token)
        } else {
            None
        }
    }

    /// Push an error onto the error list.
    fn error(&mut self, message : &'static str) {
        Error::throw(message, self.row, self.column);
    }
}


/// Additional methods for `str`
trait StrExt {
    /// Returns a substring of this `str`.
    fn substring<'a>(&'a self, i : usize, n : usize) -> &'a str;
}
impl StrExt for str {
    fn substring<'a>(&'a self, i : usize, n : usize) -> &'a str {
        let start : usize = i;
        let end : usize = if let Some((x, _)) = self
                .char_indices()
                .take(n + start)
                .next() {
            x
        } else {
            self.len()
        };
        &self[start..end]
    }
}