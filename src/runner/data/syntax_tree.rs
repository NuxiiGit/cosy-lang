#![allow(dead_code)]

use super::Position;

/// A recursive enum which stores expression information.
#[derive(Debug)]
pub enum Expr<'a> {
    Literal {
        literal : Literal
    },
    Variable {
        ident : Ident<'a>
    },
    Member {
        ident : Ident<'a>,
        expr : Box<Expr<'a>>
    },
    Call {
        ident : Ident<'a>,
        args : Vec<Expr<'a>>
    }
}

/// A struct which stores a string slice with the row and column it occurs on.
#[derive(Debug)]
pub struct Ident<'a> {
    pub ident : &'a str,
    pub position : Position
}

/// A struct which stores a `Value` with the row and column it occurs on.
#[derive(Debug)]
pub struct Literal {
    pub value : Value,
    pub position : Position
}

/// An enum which stores data type values.
#[derive(Debug)]
pub enum Value {
    Empty,
    Bool(bool),
    Char(char),
    Int(i64),
    Float(f64)
}