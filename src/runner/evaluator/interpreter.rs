#![allow(dead_code)]

use super::Value;
use super::Error;
use super::super::collections::{
    token::{ Token, TokenType },
    parse_tree::*
};

/// A struct which encapsulates the state of the evaluator.
pub struct Interpreter;
impl<'a> Interpreter {
    /// Interprets a program.
    pub fn interpret(program : Expr<'a>) -> Result<Value, Error> {
        Interpreter {}.execute(program)
    }

    /// Executes this program and returns a value.
    pub fn execute(mut self, program : Expr<'a>) -> Result<Value, Error> {
        self.touch_expression(program)
    }

    /// Evaluates an expression.
    fn touch_expression(&mut self, expr : Expr<'a>) -> Result<Value, Error> {
        match expr {
            Expr::Terminal { value } => {
                let description : &str = match value.flavour {
                    TokenType::Integer(literal) => {
                        if let Ok(value) = literal.parse::<i64>() {
                            return Ok(Value::Integer(value))
                        } else {
                            "Unable to parse integer literal"
                        }
                    },
                    _ => "Illegal token"
                };
                Err(Error {
                    description,
                    row : value.row,
                    column : value.column
                })
            },
            Expr::Member { ident, expr } => unimplemented!(),
            Expr::Call { ident, args } => unimplemented!()
        }
    }
}