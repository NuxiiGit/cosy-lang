use crate::syntax::token::*;

/// A recursive enum which stores expression information.
#[derive(Debug)]
pub enum Expr<'a> {
    Constant {
        value : Token<'a>
    },
    Variable {
        ident : Token<'a>
    },
    Member {
        expr : Box<Expr<'a>>,
        ident : Token<'a>
    },
    Call {
        func : Box<Expr<'a>>,
        arg : Box<Expr<'a>>
    },
    Tuple {
        exprs : Vec<Expr<'a>>
    }
}
impl<'a> Expr<'a> {
    /// Generates a new binary application from two arguments.
    pub fn binary_call(op : Token<'a>, left : Expr<'a>, right : Expr<'a>) -> Self {
        Expr::Call {
            func : Box::new(Expr::Call {
                func : Box::new(Expr::Variable { ident : op }),
                arg : Box::new(left)
            }),
            arg : Box::new(right)
        }
    }
}