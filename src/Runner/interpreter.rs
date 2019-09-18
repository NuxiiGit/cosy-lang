#![allow(dead_code)]

use super::essentials::{
    token::*,
    error::*,
    syntax_tree::*
};

use std::iter::Peekable;

/// A struct which encapsulates the state of the evaluator.
struct Interpreter;
impl<'a> Interpreter {
    pub fn execute(program : &Statement<'a>) -> Option<Value> {
        let mut eval : Interpreter = Interpreter {};
        eval.touch_statement(program)
    }

    fn touch_statement(&mut self, statement : &Statement<'a>) -> Option<Value> {
        None
    }
}

/// An enum which describes the different types of value.
pub enum Value {
    Char(char),
    Integer(i64),
    Float(f64)
}