#![allow(dead_code)]

use super::token::*;

/// A recursive enum which stores expression information.
#[derive(Debug)]
pub enum Expr<'a> {
    Terminal {
        value : Token<'a>
    },
    Member {
        left : Box<Expr<'a>>,
        field : Token<'a>
    },
    Call {
        ident : Token<'a>,
        args : Vec<Expr<'a>>
    }
}