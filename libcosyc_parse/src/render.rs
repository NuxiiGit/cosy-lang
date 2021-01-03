use crate::syntax as ast;
use std::fmt::{ self as fmt, Write };

/// Handles the debug rendering of the abstract syntax tree.
pub struct LispRenderer<'a, T : Write> {
    src : &'a str,
    out : T
}

impl<'a, T : Write> LispRenderer<'a, T> {
    /// Creates a new renderer from this source and output stream.
    pub fn new(src : &'a str, out : T) -> Self {
        Self { src, out }
    }

    /// Renders an expression.
    pub fn render_expr(&mut self, expr : &ast::Expr) -> fmt::Result {
        let span = &expr.span;
        match &expr.kind {
            ast::ExprKind::Variable => {
                write!(self.out, "|{}|", span.render(self.src))?
            },
            ast::ExprKind::Integral => write!(self.out, "{}", span.render(self.src))?,
            ast::ExprKind::BinaryOp { kind, lexpr, rexpr } => {
                write!(self.out, "(")?;
                match &kind {
                    ast::BinaryOpKind::Add => write!(self.out, "+")?,
                    ast::BinaryOpKind::Subtract => write!(self.out, "-")?,
                    ast::BinaryOpKind::Custom(inner) => {
                        write!(self.out, "funcall ")?;
                        self.render_expr(inner)?;
                    }
                }
                write!(self.out, " ")?;
                self.render_expr(lexpr)?;
                write!(self.out, " ")?;
                self.render_expr(rexpr)?;
                write!(self.out, ")")?;
            },
            ast::ExprKind::UnaryOp { kind, inner } => {
                write!(self.out, "(")?;
                match &kind {
                    ast::UnaryOpKind::Negate => write!(self.out, "-")?,
                }
                write!(self.out, " ")?;
                self.render_expr(inner)?;
                write!(self.out, ")")?;
            },
            ast::ExprKind::Call { intrinsic, callsite, params } => {
                write!(self.out, "funcall ")?;
                self.render_expr(callsite)?;
                if *intrinsic {
                    write!(self.out, " !")?;
                }
                for param in params {
                    write!(self.out, " ")?;
                    self.render_expr(param)?;
                }
            }
        }
        Ok(())
    }
}

/// Renders a program as Lisp pseudo-code.
pub fn render_as_lisp(src : &str, expr : &ast::Expr) -> String {
    let mut out = String::new();
    let mut renderer = LispRenderer::new(src, &mut out);
    let _ = renderer.render_expr(expr);
    out
}
