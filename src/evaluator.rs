#![allow(unused_imports)]
#![allow(dead_code)]

use crate::diagnostics::*;
use crate::syntax::{
    token::*,
    ast::*
};

use std::collections::hash_map::HashMap;

/// Takes a lexer and uses it to construct a parse tree.
pub struct Interpreter<'a> {
    environment : Environment<'a>
}

/// An enum of value types recognised by the interpreter.
#[derive(Clone)]
pub enum Value<'a> {
    Integer(i64),
    Real(f64),
    Char(char),
    Function {
        closure : Environment<'a>,
        func : fn(Environment<'a>, Value<'a>) -> Value<'a>
    }
}

/// A type alias for environments.
pub type Environment<'a> = HashMap<&'a str, Value<'a>>;