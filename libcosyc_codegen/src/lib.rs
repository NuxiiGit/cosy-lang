use libcosyc_diagnostic::{
    error::{ CompilerError, IssueTracker, Failable },
    source::Renderable
};
use libcosyc_ir::ir;
use std::fmt::Write;

const INDENTATION : &'static str = "  ";

/// Manages generation of code from IR.
pub struct Codegen<'a, W : Write> {
    src : &'a str,
    issues : &'a mut IssueTracker,
    out : W,
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
        let indent = 0;
        let newline = true;
        Self { src, issues, out, indent, newline }
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
        self.write("int result = ")?;
        self.indent();
        self.visit_c_inst(inst)?;
        self.writeln(";")?;
        self.unindent();
        self.writeln(r#"printf("%d\n", result);"#)?;
        self.unindent();
        self.write("}")?;
        Some(())
    }

    fn visit_c_inst(&mut self, inst : ir::Inst) -> Option<()> {
        let span = inst.span;
        match inst.kind {
            ir::InstKind::Value(_kind) =>
                    self.report(CompilerError::unimplemented("code generation of runtime values")
                            .span(&span))?,
            ir::InstKind::TypeAnno { .. } =>
                    self.report(CompilerError::bug()
                            .span(&span)
                            .reason("type annotations should be erased by this point"))?,
            ir::InstKind::BinaryOp { kind : _, left : _, right : _ } =>
                    self.report(CompilerError::unimplemented("code generation of binary ops")
                            .span(&span))?,
            ir::InstKind::UnaryOp { kind : _, value : _ } =>
                    self.report(CompilerError::unimplemented("code generation of unary ops")
                            .span(&span))?
        }
        Some(())
    }
}

/// Generates C code from this IR instruction.
pub fn generate_c<W : Write>(inst : ir::Inst, src : &str, issues : &mut IssueTracker, out : W) -> Option<()> {
    Codegen::new(src, issues, out).gen_c(inst)
}
