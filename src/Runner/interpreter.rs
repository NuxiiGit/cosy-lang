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
    fn execute_program(&mut self, tree : &Statement<'a>) -> Option<Value> {
        None
    }
}

/// An enum which describes the different types of value.
pub enum Value {
    Char(char),
    Integer(i64),
    Float(f64)
}