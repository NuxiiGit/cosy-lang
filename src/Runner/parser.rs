#![allow(dead_code)]

use super::lexer::Lexer;
use super::error::Error;
use super::token::*;
use super::syntax_tree::*;
use std::iter::Peekable;

macro_rules! truth {
    ($value:expr, $($condition:tt)*) => ({
        match $value {
            $($condition)* => true,
            _ => false
        }
    });
    ($($condition:tt)*) => ({
        if $($condition)* {
            true
        } else {
            false
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
        self.parse_expr_addition()
    }

    /// Parses a string of `+` and `-` binary operators.
    fn parse_expr_addition(&mut self) -> Option<Expr<'a>> {
        let mut left : Expr = self.parse_expr_frontier()?;
        while let Some(token) = self.consume_if(|x|
                truth!(x, TokenType::Operator(op) if truth!(&op[..1], "+" | "-"))) {
            let right : Expr = self.parse_expr_frontier()?;
            left = Expr::Binary(token, Box::new(left), Box::new(right));
        }
        Some(left)
    }

    /// Parses the frontier of an expression.
    fn parse_expr_frontier(&mut self) -> Option<Expr<'a>> {
        if let Some(token) = self.consume_if(|x|
                truth!(x, TokenType::String(..) |
                        TokenType::Integer(..) |
                        TokenType::Identifier(..))) {
            return Some(Expr::Terminal(token));
        }
        self.error("Malformed expression.");
        None
    }

    /// Consumes the next token if the closure returns true.
    fn consume_if(&mut self, f : impl Fn(&TokenType<'a>) -> bool) -> Option<Token<'a>> {
        let token : &Token = self.lexer.peek()?;
        if f(&token.flavour) {
            self.consume()
        } else {
            None
        }
    }

    /// Consumes the next token.
    fn consume(&mut self) -> Option<Token<'a>> {
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

/*
fn primary(tokens : &mut Tokens) -> Result<Expr, &'static str> {
    if let Some(token) = tokens.next() {
        match token.flavour() {
            TokenType::Str(x) => Ok(Expr::Str(x.to_owned())),
            TokenType::Int(x) => Ok(Expr::Int(x.to_owned())),
            TokenType::Identifier(x) => Ok(Expr::Identifier(x.to_owned())),
            TokenType::LeftParen => {
                let expr : Expr = expression(tokens)?;
                if {
                    if let Some(token) = tokens.next() {
                        if let TokenType::RightParen = token.flavour() {
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } {
                    Ok(expr)
                } else {
                    Err("Expected ')' after expression")
                match tokens.next() {
                    Some(token) if match token.flavour() {
                        TokenType::RightParen => true,
                        _ => false
                    } => Ok(expr),
                    _ => Err("Expected ')' after expression")
                }
            },
            token => {
                println!("{}", token);
                Err("Expected identifier or literal")
            }
        }
    } else {
        Err("Expected expression: Got nothing")
    }
}*/