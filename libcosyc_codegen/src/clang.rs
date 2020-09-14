use libcosyc_diagnostics::source::Span;
use libcosyc_abstract::syntax;

use std::fmt;

type Output<'a> = &'a mut dyn fmt::Write;

/// Takes an output stream and generates C code.
pub struct CCodegen<'a> {
    out : Output<'a>
}
impl<'a> From<Output<'a>> for CCodegen<'a> {
    fn from(out : Output<'a>) -> Self {
        Self { out }
    }
}
