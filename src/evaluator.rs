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
    pub fn interpret(&mut self, prog : Prog<'a>) -> Result<'a> {
        self.visit_stmt(prog.stmt)
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
    fn visit_expr_literal(&mut self, token : Token<'a>) -> Result<'a> {
        let result = if let TokenKind::Literal(kind) = &token.kind {
            let content = token.span.content;
            match kind {
                LiteralKind::Integer => {
                    if let Ok(value) = content.parse::<i64>() {
                        Ok(Value::Integer(value))
                    } else {
                        Err("unable to parse integer literal")
                    }
                },
                _ => Err("unknown literal kind")
            }
        } else {
            Err("expected literal")
        };
        match result {
            Ok(value) => Ok(value),
            Err(reason) => Err(Error { reason, token })
        }
    }
}

/// The result of the interpreter.
pub type Result<'a> = std::result::Result<Value, Error<'a>>;

/// An enum of value types recognised by the interpreter.
#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Integer(i64),
    Real(f64),
    Char(char),
    Empty
}