#![allow(dead_code)]

use super::collections::{
    token::*,
    error::*,
    syntax_tree::*
};

/// A struct which encapsulates the state of the evaluator.
pub struct Interpreter;
impl<'a> Interpreter {
    /// Constructs a new evaluator.
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    /// Executes this program and returns a value.
    pub fn execute(mut self, program : Expr<'a>) -> Result<Value, Error> {
        self.touch_expression(program)
    }

    /// Evaluates an expression.
    fn touch_expression(&mut self, expr : Expr<'a>) -> Result<Value, Error> {
        match expr {
            Expr::Terminal { value } => {
                match value.flavour {
                    TokenType::Integer(literal) => {
                        if let Ok(value) = literal.parse::<i64>() {
                            Ok(Value::Integer(value))
                        } else {
                            Err(Error::new("Unable to parse integer literal", value.row, value.column))
                        }
                    },
                    TokenType::String(literal) => unimplemented!(),
                    _ => Err(Error::new("Illegal token", value.row, value.column))
                }
            },
            Expr::Unary { operator, right } => unimplemented!(),
            Expr::Binary { operator, left, right } => unimplemented!(),
            Expr::Member { left, field } => unimplemented!()
        }
    }
}

/// An enum which describes the different types of value.
#[derive(Debug)]
pub enum Value {
    Char(char),
    Integer(i64),
    Float(f64)
}