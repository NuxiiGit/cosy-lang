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
    pub fn execute(mut self, program : &Statement<'a>) -> Option<Value> {
        self.touch_statement(program)
    }

    /// Evaluates a statement.
    fn touch_statement(&mut self, statement : &Statement<'a>) -> Option<Value> {
        match statement {
            Statement::ExpressionStatement { expr } => self.touch_expression(expr),
            _ => panic!()
        }
    }

    /// Evaluates an expression.
    fn touch_expression(&mut self, expr : &Expr<'a>) -> Option<Value> {
        match expr {
            Expr::Literal { value } => self.touch_expression_literal(value),
            /*Expr::Variable { ident } => None,
            Expr::Unary { operator, right } => None,
            Expr::Binary { operator, left, right } => None,
            Expr::Member { left, field } => None,*/
            _ => panic!()
        }
    }

    /// Evaluates a literal.
    fn touch_expression_literal(&mut self, token : &Token<'a>) -> Option<Value> {
        match token.flavour {
            TokenType::Integer(literal) => {
                if let Ok(value) = literal.parse::<i64>() {
                    Some(Value::Integer(value))
                } else {
                    None
                }
            },
            _ => panic!()
        }
    }
}

/// An enum which describes the different types of value.
pub enum Value {
    Char(char),
    Integer(i64),
    Float(f64)
}