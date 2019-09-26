#![allow(dead_code)]

use super::collections::{
    token::*,
    error::*,
    syntax_tree::*
};

/// A macro for matching a value with a pattern.
macro_rules! matches {
    ($value:expr, $($pattern:tt)*) => ({
        match $value {
            $($pattern)* => true,
            _ => false
        }
    });
}

use std::iter::Peekable;

/// A struct which encapsulates the state of the parser.
pub struct Parser<'a, I> where
        I : Iterator<Item = Token<'a>> {
    scanner : Peekable<I>,
    row : usize,
    column : usize
}
impl<'a, I> Parser<'a, I> where
        I : Iterator<Item = Token<'a>> {
    /// Constructs a new parser.
    pub fn new(scanner : I) -> Parser<'a, I> {
        Parser {
            scanner : scanner.peekable(),
            row : 0,
            column : 0
        }
    }
    
    /// Constumes the parser and produces a syntax tree of its expression.
    pub fn parse(mut self) -> Option<Statement<'a>> {
        Some(Statement::ExpressionStatement {
            expr : self.parse_expr()?
        })
    }

    /// Parses an expression.
    fn parse_expr(&mut self) -> Option<Expr<'a>> {
        let mut left : Expr = self.parse_expr_addition()?;
        while let Some(operator) = self.consume_if(|x| matches!(x, TokenType::Operator(..))) {
            let right : Expr = self.parse_expr_addition()?;
            left = Expr::Binary {
                operator : operator,
                left : Box::new(left),
                right : Box::new(right)
            };
        }
        Some(left)
    }

    /// Parses a string of `!=` and `==` binary operators.
    fn parse_expr_equality(&mut self) -> Option<Expr<'a>> {
        let mut left : Expr = self.parse_expr_inequality()?;
        while let Some(operator) = self.consume_if(|x| matches!(x, TokenType::Operator(op) if
                matches!(op.substring(0, 1), "!" | "="))) {
            let right : Expr = self.parse_expr_inequality()?;
            left = Expr::Binary {
                operator : operator,
                left : Box::new(left),
                right : Box::new(right)
            };
        }
        Some(left)
    }

    /// Parses a string of `!=` and `==` binary operators.
    fn parse_expr_inequality(&mut self) -> Option<Expr<'a>> {
        let mut left : Expr = self.parse_expr_addition()?;
        while let Some(operator) = self.consume_if(|x| matches!(x, TokenType::Operator(op) if
                matches!(op.substring(0, 1), ">" | "<"))) {
            let right : Expr = self.parse_expr_addition()?;
            left = Expr::Binary {
                operator : operator,
                left : Box::new(left),
                right : Box::new(right)
            };
        }
        Some(left)
    }

    /// Parses a string of `+` and `-` binary operators.
    fn parse_expr_addition(&mut self) -> Option<Expr<'a>> {
        let mut left : Expr = self.parse_expr_multiplication()?;
        while let Some(operator) = self.consume_if(|x| matches!(x, TokenType::Operator(op) if
                matches!(op.substring(0, 1), "+" | "-"))) {
            let right : Expr = self.parse_expr_multiplication()?;
            left = Expr::Binary {
                operator : operator,
                left : Box::new(left),
                right : Box::new(right)
            };
        }
        Some(left)
    }

    /// Parses a string of `*`, `/`, and '%' binary operators.
    fn parse_expr_multiplication(&mut self) -> Option<Expr<'a>> {
        let mut left : Expr = self.parse_expr_unary()?;
        while let Some(operator) = self.consume_if(|x| matches!(x, TokenType::Operator(op) if
                matches!(op.substring(0, 1), "*" | "/" | "%"))) {
            let right : Expr = self.parse_expr_unary()?;
            left = Expr::Binary {
                operator : operator,
                left : Box::new(left),
                right : Box::new(right)
            };
        }
        Some(left)
    }

    /// Parses any sort of chained unary operators.
    fn parse_expr_unary(&mut self) -> Option<Expr<'a>> {
        if let Some(operator) = self.consume_if(|x| matches!(x, TokenType::Operator(..))) {
            let right : Expr = self.parse_expr_unary()?;
            Some(Expr::Unary {
                operator : operator,
                right : Box::new(right)
            })
        } else {
            self.parse_expr_member()
        }
    }

    /// Parses a chain of identifiers.
    fn parse_expr_member(&mut self) -> Option<Expr<'a>> {
        let mut expr : Expr = self.parse_expr_frontier()?;
        while let Some(ident) = self.consume_if(|x| matches!(x, TokenType::Identifier(..))) {
            expr = Expr::Member {
                left : Box::new(expr),
                field : ident
            }
        }
        Some(expr)
    }

    /// Parses the frontier of an expression.
    fn parse_expr_frontier(&mut self) -> Option<Expr<'a>> {
        if let Some(literal) = self.consume_if(|x| matches!(x,
                TokenType::String(..) |
                TokenType::Integer(..))) {
            Some(Expr::Literal {
                value : literal
            })
        } else if let Some(_) = self.consume_if(|x| matches!(x, TokenType::LeftParen)) {
            let expr : Expr = self.parse_expr()?;
            if let Some(_) = self.consume_if(|x| matches!(x, TokenType::RightParen)) {
                Some(expr)
            } else {
                self.error("Expected ending ')' after expression");
                None
            }
        } else {
            self.error("Malformed expression");
            None
        }
    }

    /// Consumes the next token if the closure returns true.
    fn consume_if(&mut self, f : impl Fn(&TokenType<'a>) -> bool) -> Option<Token<'a>> {
        let token : &Token = self.scanner.peek()?;
        if f(&token.flavour) {
            self.consume_next()
        } else {
            None
        }
    }

    /// Consumes the next token.
    fn consume_next(&mut self) -> Option<Token<'a>> {
        if let Some(token) = self.scanner.next() {
            self.row = token.row;
            self.column = token.column;
            Some(token)
        } else {
            None
        }
    }

    /// Push an error onto the error list.
    fn error(&mut self, message : &'static str) {
        Error::new(message, self.row, self.column).throw();
    }
}

/// Implement `into_ast()` methods onto all iterators where their item is `token::Token`.
pub trait IteratorExt<'a> : Iterator<Item = Token<'a>> {
    /// Consumes this iterator and converts it into a parse tree of tokens.
    fn into_ast(self) -> Option<Statement<'a>>;
}
impl<'a, I> IteratorExt<'a> for I where 
        I : Iterator<Item = Token<'a>> {
    fn into_ast(self) -> Option<Statement<'a>> {
        Parser::new(self).parse()
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