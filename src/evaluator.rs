#![allow(unused_imports)]
#![allow(dead_code)]

use crate::diagnostics::*;
use crate::syntax::{
    token::*,
    ast::*
};

/// Takes a lexer and uses it to construct a parse tree.
pub struct Interpreter;
impl Interpreter {
    
}

/// An enum of value types recognised by the interpreter.
#[derive(Clone)]
pub enum Value {
    Integer(i64),
    Real(f64),
    Char(char)
}