#![allow(dead_code)]

use super::essentials::{
    token::*,
    error::*,
    syntax_tree::*
};

use std::iter::Peekable;

/// A struct which encapsulates the state of the evaluator.
struct Interpreter<'a> {
    program : Statement<'a>
}
impl<'a> Interpreter<'a> {
    /// Constructs a new evaluator.
    pub fn new(program : Statement<'a>) -> Interpreter<'a> {
        Interpreter {
            program
        }
    }

    pub fn execute(mut self) -> Option<Value> {
        None
    }
}

/// An enum which describes the different types of value.
pub enum Value {
    Char(char),
    Integer(i64),
    Float(f64)
}