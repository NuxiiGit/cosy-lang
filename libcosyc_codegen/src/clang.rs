use libcosyc_diagnostics::Span;
use libcosyc_abstract::syntax;

use std::fmt;

/// Represents any kind of output stream.
pub type Output<'a> = &'a mut dyn fmt::Write;

/// Provides an interface for generating C code.
pub trait CGen {
    fn codegen(self, out : Output) -> fmt::Result;
}
