use libcosyc_diagnostic::{
    error::{ CompilerError, IssueTracker, Failable },
    source::Renderable
};
use libcosyc_ir::ir;

/// Manages generation of code from IR.
pub struct Codegen<'a> {
    src : &'a str,
    issues : &'a mut IssueTracker
}

impl Failable for Codegen<'_> {
    fn issues(&mut self) -> &mut IssueTracker {
        self.issues
    }
}

impl Renderable for Codegen<'_> {
    fn src(&self) -> &str {
        self.src
    }
}

impl Codegen<'_> {

}
