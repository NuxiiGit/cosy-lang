use libcosyc_diagnostics::source::Span;
use libcosyc_abstract::syntax::*;

use std::fmt;

type Output<'a> = &'a mut dyn fmt::Write;

/// Provides an interface for generating C code from abstract syntax.
pub trait Codegen {
    /// Write code to output stream `out`.
    fn codegen(self, out : Output);
}

impl Codegen for Expr {
    fn codegen(self, out : Output) {
        unimplemented!()
    }
}
