#![allow(unused_imports)]

use crate::diagnostics::*;
use crate::syntax::{
    token::*,
    ast::*
};

use std::collections::hash_map::HashMap;

/// Takes a lexer and uses it to construct a parse tree.
pub struct Interpreter {
}

/// An enum of value types recognised by the interpreter.
#[derive(Clone)]
pub enum Value {
    Integer(i64),
    Real(f64),
    Char(char),
    Function {
        capture : Box<Value>,
        func : fn(Value, Value) -> Value
    }
}