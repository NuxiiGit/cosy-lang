#![allow(dead_code)]

use super::token::*;

/// An enum which stores the root of the syntax tree.
#[derive(Debug)]
pub enum SyntaxTree<'a> {
    Statement {
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
        expr : Box<Expr<'a>>
    },
    Binary {
        operator : Token<'a>,
        left : Box<Expr<'a>>,
        right : Box<Expr<'a>>
    },
    Member {
        expr : Box<Expr<'a>>,
        field : Token<'a>
    },
}