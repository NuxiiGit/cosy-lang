use crate::syntax::token::*;

use std::fmt;

macro_rules! write_op {
    ($out:expr, $ident:expr) => (if $ident.is_op() {
        write!($out, "({})", $ident)
    } else {
        write!($out, "{}", $ident)
    })
}

/// A struct which encapsulates information about a program.
#[derive(Debug)]
pub struct Prog<'a> {
    pub stmts : Vec<Stmt<'a>>
}
impl fmt::Display for Prog<'_> {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "{}", self.stmts.iter().fold(String::new(), |mut acc, stmt| {
            acc.push_str(&stmt.to_string());
            acc.push('\n');
            acc
        }))
    }
}

/// A recursive enum which stores statement information.
#[derive(Debug)]
pub enum Stmt<'a> {
    Expr {
        expr : Expr<'a>
    },
    Declr {
        ident : Token<'a>,
        expr : Expr<'a>
    },
    Block {
        stmts : Vec<Stmt<'a>>
    }
}
impl fmt::Display for Stmt<'_> {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stmt::Expr { expr } => write!(out, "{};", expr),
            Stmt::Block { stmts } => {
                write!(out, "{{\n{}\n}}", stmts.iter().fold(String::new(), |mut acc, stmt| {
                    acc.push_str(&stmt.to_string());
                    acc.push('\n');
                    acc
                }))
            },
            _ => unimplemented!()
        }
    }
}

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
    Lambda {
        param : Token<'a>,
        body : Box<Expr<'a>>
    },
    Tuple {
        exprs : Vec<Expr<'a>>
    }
}
impl<'a> Expr<'a> {
    /// Generates a new binary application from two arguments.
    pub fn binary_call(op : Expr<'a>, left : Expr<'a>, right : Expr<'a>) -> Self {
        Expr::Call {
            func : Box::new(Expr::Call {
                func : Box::new(op),
                arg : Box::new(left)
            }),
            arg : Box::new(right)
        }
    }

    /// Returns whether this expression is an operator identifier.
    pub fn is_op(&self) -> bool {
        if let Expr::Variable { ident } = self {
            if let TokenKind::Identifier(IdentifierKind::Operator) = ident.kind {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Returns whther this expression is a unary operation.
    pub fn is_unaryop(&self) -> bool {
        if let Expr::Call { func, .. } = self {
            func.is_op()
        } else {
            false
        }
    }

    /// Returns whther this expression is a binary operation.
    pub fn is_binaryop(&self) -> bool {
        if let Expr::Call { func, .. } = self {
            func.is_unaryop()
        } else {
            false
        }
    }
}
impl fmt::Display for Expr<'_> {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        if self.is_binaryop() {
            if let Expr::Call { func, arg : a } = self {
                if let Expr::Call { func : op, arg : b } = &**func { // ???
                    write!(out, "(")?;
                    write_op!(out, b)?;
                    write!(out, " {} ", op)?;
                    write_op!(out, a)?;
                    write!(out, ")")
                } else {
                    unreachable!()
                }
            } else {
                unreachable!()
            }
        } else if self.is_unaryop() {
            if let Expr::Call { func, arg } = self {
                write!(out, "{}", func)?;
                write_op!(out, arg)
            } else {
                unreachable!()
            }
        } else {
            match self {
                Expr::Constant { value } => write!(out, "{}", value),
                Expr::Variable { ident } => write!(out, "{}", ident),
                Expr::Member { expr, ident } => write!(out, "{}.{}", expr, ident),
                Expr::Call { func, arg } => {
                    write!(out, "(")?;
                    write_op!(out, func)?;
                    write!(out, " ")?;
                    write_op!(out, arg)?;
                    write!(out, ")")
                },
                Expr::Lambda { param, body } => write!(out, "(\\{} -> {})", param, body),
                Expr::Tuple { exprs } => {
                    let tuple = exprs.iter().fold(String::new(), |mut acc, expr| {
                        if !acc.is_empty() {
                            acc.push(',');
                        }
                        acc.push_str(&expr.to_string());
                        acc
                    });
                    write!(out, "({})", tuple)
                }
            }
        }
    }
}