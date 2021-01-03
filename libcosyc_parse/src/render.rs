use crate::syntax as ast;
use std::fmt::{ self as fmt, Write };

fn valid_identifier(ident : &str) -> bool {
    if matches!(ident,
            "min"
            | "max"
            | "defun"
            | "equal"
            | "equalp"
            | "eq"
            | "eql"
            | "eval"
            | "setq"
            | "setf"
            | "set"
            | "let"
            | "flet"
            | "labels"
            | "defmethod"
            | "defvar"
            | "defparameter"
            | "defsetf"
            | "lambda"
            | "funcall"
            | "apply"
            | "return"
            | "identity"
            | "go"
            | "throw"
            | "error"
            | "signal"
            | "cerror"
            | "warn"
            | "if"
            | "cond"
            | "loop"
            | "do"
            | "until"
            | "with"
            | "while"
            | "for"
            | "from"
            | "to"
            | "by"
            | "case"
            | "otherwise"
            | "declare"
            | "deftype"
            | "defclass"
            | "defstruct"
            | "dispatching"
            | "method"
            | "typep"
            | "defpackage"
            | "export"
            | "require"
            | "import"
            | "char"
            | "aref"
            | "schar"
            | "svref"
            | "coerce"
            | "subseq"
            | "search"
            | "write"
            | "print"
            | "princ"
            | "format"
            | "concatenate"
            | "length"
            | "nil"
            | "not"
            | "or"
            | "and"
            | "t"
            | "boolean"
            | "cons"
            | "push"
            | "cdr"
            | "reduce"
            | "find"
            | "car"
            | "dolist"
            | "pop"
            | "member"
            | "some"
            | "every"
            | "upfrom"
            | "as"
            | "last"
            | "nconc"
            | "append"
            | "list"
            | "pairlis"
            | "nth"
            | "assoc"
            | "reverse"
            | "sort"
            | "mapcar"
            | "gethash"
            | "remhash"
            | "v"
            | "logand"
            | "logior"
            | "logxor"
            | "lognot"
            | "ash"
            | "floor"
            | "expt"
            | "log"
            | "mod"
            | "random"
            | "sqrt"
            | "exp"
            | "abs"
            | "sin"
            | "cos"
            | "tan"
            | "asin"
            | "acos"
            | "atan"
            | "truncate"
            | "round"
            | "ceiling") {
        return false;
    }
    let mut chars = ident.chars();
    match chars.next() {
        Some(x) => {
            if !x.is_alphabetic() && !matches!(x, '_') {
                return false;
            }
        }
        None => return false
    }
    chars.all(|x| x.is_alphanumeric())
}

/// Handles the debug rendering of the abstract syntax tree.
pub struct LispRenderer<'a, T : Write> {
    src : &'a str,
    out : T,
    indent : usize
}

impl<'a, T : Write> LispRenderer<'a, T> {
    /// Creates a new renderer from this source and output stream.
    pub fn new(src : &'a str, out : T) -> Self {
        let indent = 0;
        Self { src, out, indent }
    }

    fn indent(&mut self) {
        self.indent += 1;
    }

    fn unindent(&mut self) {
        self.indent -= 1;
    }

    fn newline(&mut self) -> fmt::Result {
        write!(self.out, "\n{}", "  ".repeat(self.indent))
    }

    fn render_expr_params(&mut self, inline : bool, params : &[&ast::Expr]) -> fmt::Result {
        if !inline {
            self.indent();
        }
        for param in params {
            if inline {
                write!(self.out, " ")?;
            } else {
                self.newline()?;
            }
            self.render_expr(param)?;
        }
        if !inline {
            self.unindent();
        }
        Ok(())
    }

    /// Renders an expression.
    pub fn render_expr(&mut self, expr : &ast::Expr) -> fmt::Result {
        let span = &expr.span;
        match &expr.kind {
            ast::ExprKind::Variable => {
                let ident = span.render(self.src);
                if valid_identifier(ident) {
                    write!(self.out, "{}", ident)?
                } else {
                    write!(self.out, "|{}|", ident)?
                }
            },
            ast::ExprKind::Integral => write!(self.out, "{}", span.render(self.src))?,
            ast::ExprKind::BinaryOp { kind, lexpr, rexpr } => {
                write!(self.out, "(")?;
                let mut inline = true;
                match &kind {
                    ast::BinaryOpKind::Add => write!(self.out, "+")?,
                    ast::BinaryOpKind::Subtract => write!(self.out, "-")?,
                    ast::BinaryOpKind::Custom(inner) => {
                        write!(self.out, "funcall ")?;
                        self.render_expr(inner)?;
                        inline = false;
                    }
                }
                self.render_expr_params(inline, &[lexpr, rexpr])?;
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
                write!(self.out, "(funcall ")?;
                self.render_expr(callsite)?;
                if *intrinsic {
                    write!(self.out, " :intrinsic")?;
                }
                let params : Vec<&ast::Expr> = params.iter().collect();
                self.render_expr_params(false, &params)?;
                write!(self.out, ")")?;
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
