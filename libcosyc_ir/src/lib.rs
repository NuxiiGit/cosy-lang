pub mod ir;
pub mod desugar;
pub mod typecheck;

use libcosyc_diagnostic::error::IssueTracker;
use libcosyc_parse::syntax as ast;

/// Applies semantic analysis to this AST and returns valid IR.
pub fn generate_ir(ast : ast::Term, src : &str, issues : &mut IssueTracker) -> Option<ir::Inst> {
    let inst = desugar::surface_into_core(ast, src, issues)?;
    Some(inst)
}
