#![allow(dead_code)]

use super::token::*;

/// An enum which stores statement information.
#[derive(Debug)]
pub enum Statement<'a> {
    ExpressionStatement {
        expr : Expr<'a>
    }
}

/// A recursive enum which stores expression information.
#[derive(Debug)]
pub enum Expr<'a> {
    Literal {
        value : Token<'a>
    },
    Variable {
        ident : Token<'a>
    },
    Unary {
        operator : Token<'a>,
        right : Box<Expr<'a>>
    },
    Binary {
        operator : Token<'a>,
        left : Box<Expr<'a>>,
        right : Box<Expr<'a>>
    },
    Member {
        left : Box<Expr<'a>>,
        field : Token<'a>
    },
}