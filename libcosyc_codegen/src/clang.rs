use libcosyc_diagnostics::source::Span;
use libcosyc_abstract::syntax::*;

use std::fmt;

/// Represents the state of the C code generator.
pub struct Codegen<'a> {
    out : &'a mut dyn fmt::Write
}
impl<'a> From<&'a mut dyn fmt::Write> for Codegen<'a> {
    fn from(out : &'a mut dyn fmt::Write) -> Self {
        Self { out }
    }
}
