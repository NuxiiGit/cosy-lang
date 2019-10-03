#![allow(dead_code)]

use super::token::Token;

/// A recursive enum which stores expression information.
#[derive(Debug)]
pub enum Expr<'a> {
    Terminal {
        value : Token<'a>
    },
    Member {
        ident : Token<'a>,
        expr : Box<Expr<'a>>
    },
    Call {
        ident : Token<'a>,
        args : Vec<Expr<'a>>
    }
}