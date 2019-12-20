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
    pub fn interpret(&mut self, prog : &Prog<'a>) -> Value {
        self.visit_stmt(&prog.stmt)
    }

    /// Visits a statement.
    fn visit_stmt(&mut self, stmt : &Stmt<'a>) -> Value {
        match stmt {
            Stmt::Expr { expr } => self.visit_expr(&expr),
            Stmt::Block { stmts } => {
                if stmts.len() == 0 {
                    Value::Empty
                } else {
                    self.visit_stmt(&stmts[0])
                }
            }
        }
    }

    /// Visits an expression.
    fn visit_expr(&mut self, expr : &Expr<'a>) -> Value {
        match expr {
            Expr::Constant { value } => self.visit_expr_literal(&value),
            Expr::Variable { ident } => unimplemented!(),
            _ => unimplemented!()
        }
    }

    /// Visits a literal.
    fn visit_expr_literal(&mut self, literal : &Token<'a>) -> Value {
        if let TokenKind::Literal(kind) = &literal.kind {
            let content = literal.span.content;
            match kind {
                LiteralKind::Integer => {
                    if let Ok(value) = content.parse::<i64>() {
                        Value::Integer(value)
                    } else {
                        panic!("invalid parse")
                    }
                },
                _ => unimplemented!()
            }
        } else {
            panic!("temp error")
        }
    }
}

/// An enum of value types recognised by the interpreter.
#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Integer(i64),
    Real(f64),
    Char(char),
    Empty
}