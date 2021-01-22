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
    indent : usize
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
        Self { src, issues, out, indent }
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
        match write!(self.out, "{}", string.to_string()) {
            Ok(()) => Some(()),
            Err(e) => self.report(CompilerError::bug().reason(e))?
        }
    }

    /// Writes a newline to the output.
    pub fn writeln(&mut self, count : usize) -> Option<()> {
        let mut lines = "\n".repeat(count);
        lines.push_str(&INDENTATION.repeat(self.indent));
        self.write(lines)
    }

    /// Consumes this code generator and writes the C code for this IR instruction.
    pub fn gen_c(mut self, inst : ir::Inst) -> Option<()> {
        self.write("#include <stdio.h>")?;
        self.writeln(2)?;
        self.write("int main() {")?;
        self.indent();
        self.writeln(1)?;
        self.write("int result = ")?;
        self.indent();
        self.visit_c_inst(inst)?;
        self.write(";")?;
        self.unindent();
        self.writeln(1)?;
        self.write(r#"printf("%d\n", result);"#)?;
        self.unindent();
        self.writeln(1)?;
        self.write("}")?;
        Some(())
    }

    fn visit_c_inst(&mut self, _inst : ir::Inst) -> Option<()> {
        self.write("hello")?;
        self.writeln(1)?;
        self.write("world")?;
        Some(())
    }
}

/// Generates C code from this IR instruction.
pub fn generate_c<W : Write>(inst : ir::Inst, src : &str, issues : &mut IssueTracker, out : W) -> Option<()> {
    Codegen::new(src, issues, out).gen_c(inst)
}
