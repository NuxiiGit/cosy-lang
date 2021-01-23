use libcosyc_diagnostic::{
    error::{ CompilerError, IssueTracker, Failable },
    source::Renderable
};
use libcosyc_ir::ir;
use std::fmt::Write;

/// Returns the c representation of a local variable with this index.
fn c_local(local : usize) -> String {
    format!("t{}", local)
}

const INDENTATION : &'static str = "  ";

/// Manages generation of code from IR.
pub struct Codegen<'a, W : Write> {
    src : &'a str,
    issues : &'a mut IssueTracker,
    out : W,
    next_local : usize,
    indent : usize,
    newline : bool
}

impl<W : Write> Failable for Codegen<'_, W> {
    fn issues(&mut self) -> &mut IssueTracker {
        self.issues
    }
}

impl<W : Write> Renderable for Codegen<'_, W> {
    fn src(&self) -> &str {
        self.src
    }
}

impl<'a, W : Write> Codegen<'a, W> {
    /// Creates a new instance from this issue tracker and source file.
    pub fn new(src : &'a str, issues : &'a mut IssueTracker, out : W) -> Self {
        let next_local = 0;
        let indent = 0;
        let newline = true;
        Self { src, issues, out, next_local, indent, newline }
    }

    /// Returns the next local id.
    pub fn get_next_local(&mut self) -> usize {
        let local = self.next_local;
        self.next_local += 1;
        local
    }

    /// Increases the indentation of the output.
    pub fn indent(&mut self) {
        self.indent += 1;
    }

    /// Decreases the indentation of the output.
    pub fn unindent(&mut self) {
        self.indent -= 1;
    }

    /// Writes a formatted string to the output stream.
    /// # Errors
    /// Returns `None` if there was a formatting error.
    pub fn write<T : ToString>(&mut self, string : T) -> Option<()> {
        let mut indent = String::new();
        if self.newline {
            self.newline = false;
            indent.push_str(&INDENTATION.repeat(self.indent));
        }
        match write!(self.out, "{}{}", indent, string.to_string()) {
            Ok(()) => Some(()),
            Err(e) => self.report(CompilerError::bug().reason(e))?
        }
    }

    /// Writes a newline to the output.
    pub fn writeln<T : ToString>(&mut self, string : T) -> Option<()> {
        self.write(string)?;
        self.write("\n")?;
        self.newline = true;
        Some(())
    }

    /// Consumes this code generator and writes the C code for this IR instruction.
    pub fn gen_c(mut self, inst : ir::Inst) -> Option<()> {
        self.writeln("#include <stdio.h>")?;
        self.writeln("int main() {")?;
        self.indent();
        let local = self.visit_c_inst(inst)?;
        self.write(r#"printf("%d\n", "#)?;
        self.write(c_local(local))?;
        self.writeln(r#");"#)?;
        self.writeln("return 0;")?;
        self.unindent();
        self.write("}")
    }

    fn visit_c_type(&mut self, ty : ir::TypeKind) -> Option<()> {
        match ty {
            ir::TypeKind::I8 => self.write("signed char"),
            ir::TypeKind::TypeUniverse(_) => self.report(CompilerError::unreachable("type universes"))?,
            ir::TypeKind::Unknown => self.report(CompilerError::unreachable("unknown types"))?
        }
    }

    fn visit_c_inst(&mut self, inst : ir::Inst) -> Option<usize> {
        let span = inst.span;
        let rvalue = match inst.kind {
            ir::InstKind::Value(_kind) => self.render(&span).to_string(),
            ir::InstKind::TypeAnno { .. } =>
                    self.report(CompilerError::unreachable("code generation of type ascriptions")
                            .span(&span))?,
            ir::InstKind::BinaryOp { kind, left, right } => {
                let a = self.visit_c_inst(*left)?;
                let b = self.visit_c_inst(*right)?;
                let op = match kind {
                    ir::BinaryOpKind::Add => "+",
                    ir::BinaryOpKind::Subtract => "-"
                };
                format!("{} {} {}", c_local(a), op, c_local(b))
            },
            ir::InstKind::UnaryOp { kind, value } => {
                let x = self.visit_c_inst(*value)?;
                let op = match kind {
                    ir::UnaryOpKind::Negate => "-"
                };
                format!("{}{}", op, c_local(x))
            }
        };
        let local = self.get_next_local();
        self.visit_c_type(inst.datatype)?;
        self.write(" ")?;
        self.write(c_local(local))?;
        self.write(" = ")?;
        self.write(rvalue)?;
        self.writeln(";")?;
        Some(local)
    }
}

/// Generates C code from this IR instruction.
pub fn generate_c<W : Write>(inst : ir::Inst, src : &str, issues : &mut IssueTracker, out : W) -> Option<()> {
    Codegen::new(src, issues, out).gen_c(inst)
}
