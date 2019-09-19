#![allow(dead_code)]

use super::essentials::{
    token::*,
    error::*,
    syntax_tree::*
};

/// A struct which encapsulates the state of the evaluator.
struct Interpreter;
impl<'a> Interpreter {
    /// Constructs a new evaluator.
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    /// Executes this program and returns a value.
    pub fn execute(mut self, program : &Statement<'a>) -> Result<Value, Error> {
        self.touch_statement(program)
    }

    /// Evaluates a statement.
    fn touch_statement(&mut self, statement : &Statement<'a>) -> Result<Value, Error> {
        match statement {
            Statement::ExpressionStatement { expr } => self.touch_expression(expr),
            _ => unimplemented!()
        }
    }

    /// Evaluates an expression.
    fn touch_expression(&mut self, expr : &Expr<'a>) -> Result<Value, Error> {
        match expr {
            Expr::Literal { value } => self.touch_expression_literal(value),
            _ => unimplemented!()
        }
    }

    /// Evaluates a literal.
    fn touch_expression_literal(&mut self, token : &Token<'a>) -> Result<Value, Error> {
        match token.flavour {
            TokenType::Integer(literal) => {
                if let Ok(value) = literal.parse::<i64>() {
                    Ok(Value::Integer(value))
                } else {
                    Err("Unable to parse integer literal", token.row, token.column))
                }
            },
            _ => unimplemented!()
        }
    }
}

/// An enum which describes the different types of value.
pub enum Value {
    Char(char),
    Integer(i64),
    Float(f64)
}