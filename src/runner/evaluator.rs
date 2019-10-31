#![allow(dead_code)]

use super::data::{
    Position,
    error::*,
    syntax_tree::*
};

/// A struct which interprets an abstract syntax tree.
pub struct Interpreter;
impl<'a> Interpreter {
    /// Creates a new interpreter.
    pub fn new() -> Self {
        Interpreter {}
    }

    /// Consumes this interpreter and syntx tree produces a value.
    pub fn execute(mut self, program : Expr<'a>) -> Result<Value, Error> {
        let Literal { value, .. } = self.touch_expression(program)?;
        Ok(value)
    }

    /// Evaluates an expression.
    fn touch_expression(&mut self, expr : Expr<'a>) -> Result<Literal, Error> {
        match expr {
            Expr::Literal { literal } => Ok(literal),
            Expr::Variable { ident } => unimplemented!(),
            Expr::Member { ident, expr } => unimplemented!(),
            Expr::Call { ident, args } => {
                match ident {
                    Ident { ident : "+", position } => {
                        let mut sum = 0;
                        for operand in args {
                            let Literal { value, position } = self.touch_expression(operand)?;
                            if let Value::Int(n) = value {
                                sum += n;
                            } else {
                                return Err(Error { description : "Expected Int", position });
                            }
                        }
                        Ok(Literal { value : Value::Int(sum), position })
                    }
                    Ident { position, .. } => Err(Error { description : "Unsupported operator", position })
                }
            }
        }
    }
}