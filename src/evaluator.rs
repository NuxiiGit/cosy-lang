use crate::diagnostics::*;
use crate::syntax::{
    token::*,
    ast::*
};

/// Takes a lexer and uses it to construct a parse tree.
pub struct Interpreter {}
impl<'a> Interpreter {
    /// Creates a new interpreter.
    pub fn new() -> Self {
        Interpreter {}
    }

    /// Interprets some syntax tree.
    pub fn interpret(&mut self, mut prog : Prog<'a>) -> Result<'a> {
        if prog.stmts.len() == 0 {
            Ok(Value::Empty)
        } else {
            self.visit_stmt(prog.stmts.remove(0))
        }
    }

    /// Visits a statement.
    fn visit_stmt(&mut self, stmt : Stmt<'a>) -> Result<'a> {
        match stmt {
            Stmt::Expr { expr } => self.visit_expr(expr),
            Stmt::Block { mut stmts } => {
                if stmts.len() == 0 {
                    Ok(Value::Empty)
                } else {
                    self.visit_stmt(stmts.remove(0))
                }
            }
        }
    }

    /// Visits an expression.
    fn visit_expr(&mut self, expr : Expr<'a>) -> Result<'a> {
        match expr {
            Expr::Constant { value } => self.visit_expr_literal(value),
            Expr::Variable { ident } => unimplemented!(),
            _ => unimplemented!()
        }
    }

    /// Visits a literal.
    fn visit_expr_literal(&mut self, literal : Token<'a>) -> Result<'a> {
        let error_msg = if let TokenKind::Literal(kind) = &literal.kind {
            let content = literal.span.content;
            match kind {
                LiteralKind::Integer => {
                    if let Ok(value) = content.parse::<i64>() {
                        return Ok(Value::Integer(value));
                    } else {
                        "unable to parse integer literal"
                    }
                },
                LiteralKind::Real => {
                    if let Ok(value) = content.parse::<f64>() {
                        return Ok(Value::Real(value));
                    } else {
                        "unable to parse float literal"
                    }
                },
                LiteralKind::Character => {
                    let mut indices = content.char_indices();
                    indices.next();
                    if let Some((start, _)) = indices.next() {
                        if let Some((end, _)) = indices.next_back() {
                            if let Ok(value) = (&content[start..end]).parse::<char>() {
                                return Ok(Value::Character(value));
                            }
                        }
                    }
                    "unable to parse character literal"
                },
                _ => "unknown literal kind"
            }
        } else {
            "expected literal"
        };
        Err(Error {
            reason : error_msg,
            token : literal
        })
    }
}

/// The result of the interpreter.
pub type Result<'a> = std::result::Result<Value, Error<'a>>;

/// An enum of value types recognised by the interpreter.
#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Integer(i64),
    Real(f64),
    Character(char),
    Empty
}