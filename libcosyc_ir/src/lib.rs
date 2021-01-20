pub mod ir;
pub mod eval;
pub mod typecheck;
pub mod desugar;

use libcosyc_diagnostic::error::IssueTracker;
use libcosyc_parse::syntax as ast;
use desugar::ASTDesugar;

/// Desugars the AST into IR and reports any errors to this `IssueTracker`.
pub fn desugar_ast(ast : ast::Term, src : &str, issues : &mut IssueTracker) -> Option<ir::Inst> {
    let mut desugar = ASTDesugar::new(src, issues);
    desugar.visit(ast)
}

/// Computes the constant terms of this program and returns the new program.
pub fn fold_const_terms(_inst : ir::Inst, _src : &str, _issues : &mut IssueTracker) -> Option<ir::Inst> {
    unimplemented!()
}

/// Typechecks the program and reports and type errors to this `IssueTracker`.
pub fn typecheck(_inst : &ir::Inst, _src : &str, _issues : &mut IssueTracker) -> Option<()> {
    unimplemented!()
}
