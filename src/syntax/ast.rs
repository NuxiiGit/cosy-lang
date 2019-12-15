use crate::syntax::token::*;

use std::fmt;

/// A struct which encapsulates information about a program.
#[derive(Debug)]
pub struct Program<'a> {
    pub stmts : Vec<Statement<'a>>
}
impl fmt::Display for Program<'_> {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        let prog = self.stmts.iter().fold(String::new(), |mut acc, stmt| {
            if !acc.is_empty() {
                acc.push('\n');
            }
            acc.push_str(&stmt.to_string());
            acc
        });
        write!(out, "{}", prog)
    }
}

/// A recursive enum which stores statement information.
#[derive(Debug)]
pub enum Statement<'a> {
    ExprStmt {
        expr : Expr<'a>
    }
}
impl fmt::Display for Statement<'_> {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::ExprStmt { expr } => write!(out, "{};", expr)
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

    fn __binary_right(&self) -> String {
        if let Expr::Call { func, arg } = self {
            format!("{} {}", func, arg)
        } else {
            unreachable!()
        }
    }
}
impl fmt::Display for Expr<'_> {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        if self.is_binaryop() {
            if let Expr::Call { func, arg : a } = self {
                match *func {
                    Expr::Call { func : op, arg : b } => {
                        write!(out, "({} {} {})", a, op, b)
                    },
                    _ => unreachable!()
                }
            } else {
                unreachable!()
            }
        } else if self.is_unaryop() {
            if let Expr::Call { func, arg } = self {
                write!(out, "({}{})", func, arg)
            } else {
                unreachable!()
            }
        } else {
            match self {
                Expr::Constant { value } => write!(out, "{}", value),
                Expr::Variable { ident } => write!(out, "{}", ident),
                Expr::Member { expr, ident } => write!(out, "{}.{}", expr, ident),
                Expr::Call { func, arg } => write!(out, "({} {})", func, arg),
                Expr::Tuple { exprs } => {
                    let tuple = exprs.iter().fold(String::new(), |mut acc, expr| {
                        if !acc.is_empty() {
                            acc.push(',');
                        }
                        acc.push_str(&expr.to_string());
                        acc
                    });
                    write!(out, "({})", tuple)
                },
            }
        }
    }
}