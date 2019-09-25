#![allow(dead_code)]

use super::essentials::{
    token::*,
    error::*,
    syntax_tree::*,
    macros::*
};

/// A struct which encapsulates the state of the evaluator.
pub struct Interpreter;
impl<'a> Interpreter {
    /// Constructs a new evaluator.
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    /// Executes this program and returns a value.
    pub fn execute(mut self, program : Statement<'a>) -> Result<Value, Error> {
        self.touch_statement(program)
    }

    /// Evaluates a statement.
    fn touch_statement(&mut self, statement : Statement<'a>) -> Result<Value, Error> {
        match statement {
            Statement::ExpressionStatement { expr } => self.touch_expression(expr)
        }
    }

    /// Evaluates an expression.
    fn touch_expression(&mut self, expr : Expr<'a>) -> Result<Value, Error> {
        match expr {
            Expr::Literal { value } => {
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
            Expr::Variable { ident } => unimplemented!(),
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