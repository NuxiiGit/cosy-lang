pub mod ir;
pub mod desugar;

use libcosyc_diagnostic::error::IssueTracker;
use libcosyc_parse::syntax as ast;
use desugar::ASTDesugar;

/// Desugars the AST into IR and reports any errors to this `IssueTracker`.
pub fn desugar_ast(ast : ast::Term, src : &str, issues : &mut IssueTracker) -> Option<ir::Inst> {
    let mut desugar = ASTDesugar::new(src, issues);
    desugar.visit(ast)
}
