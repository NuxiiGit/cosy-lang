use libcosyc_diagnostic::{
    error::{ CompilerError, IssueTracker, Failable },
    source::Renderable
};
use libcosyc_ir::ir;
use std::{ fmt::Write, collections::HashMap };

const INDENTATION : &'static str = "  ";

/// Manages generation of code from IR.
pub struct Codegen<'a, W : Write> {
    src : &'a str,
    issues : &'a mut IssueTracker,
    out : W,
    locals : HashMap<&'a str, usize>,
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
        let locals = HashMap::new();
        let indent = 0;
        let newline = true;
        Self { src, issues, out, locals, next_local, indent, newline }
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

    /// Writes a local variale to the output.
    fn write_local(&mut self, local : usize) -> Option<()> {
        self.write("t")?;
        self.write(local)
    }

    /// Consumes this code generator and writes the C code for this IR instruction.
    pub fn gen_c(mut self, inst : ir::Inst) -> Option<()> {
        self.writeln("#include <stdio.h>")?;
        self.writeln("int main() {")?;
        self.indent();
        let local = self.visit_c_inst(inst)?;
        self.write("int result = ")?;
        self.write_local(local)?;
        self.writeln(";")?;
        self.writeln(r#"printf("%d\n", result);"#)?;
        self.writeln("return 0;")?;
        self.unindent();
        self.write("}")
    }

    fn visit_c_local(&mut self) -> Option<usize> {
        let local = self.next_local;
        self.next_local += 1;
        self.write("int ")?;
        self.write_local(local)?;
        self.writeln(";")?;
        Some(local)
    }

    fn visit_c_inst(&mut self, inst : ir::Inst) -> Option<usize> {
        let span = inst.span;
        let local = self.visit_c_local()?;
        match inst.kind {
            ir::InstKind::Value(kind) => {
                let val = self.render(&span).to_string();
                self.write_local(local)?;
                self.write(" = ")?;
                self.write(val)?;
                self.writeln(";")?;
            }
            ir::InstKind::TypeAnno { .. } =>
                    self.report(CompilerError::unreachable("code generation of type ascriptions")
                            .span(&span))?,
            ir::InstKind::BinaryOp { kind, left, right } => {
                let a = self.visit_c_inst(*left)?;
                let b = self.visit_c_inst(*right)?;
                self.write_local(local)?;
                self.write(" = ")?;
                self.write_local(a)?;
                match kind {
                    ir::BinaryOpKind::Add => self.write(" + ")?,
                    ir::BinaryOpKind::Subtract => self.write(" - ")?
                }
                self.write_local(b)?;
                self.writeln(";")?;
            },
            ir::InstKind::UnaryOp { kind, value } => {
                let x = self.visit_c_inst(*value)?;
                self.write_local(local)?;
                self.write(" = ")?;
                match kind {
                    ir::UnaryOpKind::Negate => self.write("-")?
                }
                self.write_local(x)?;
                self.writeln(";")?;
            }
        }
        Some(local)
    }
}

/// Generates C code from this IR instruction.
pub fn generate_c<W : Write>(inst : ir::Inst, src : &str, issues : &mut IssueTracker, out : W) -> Option<()> {
    Codegen::new(src, issues, out).gen_c(inst)
}
