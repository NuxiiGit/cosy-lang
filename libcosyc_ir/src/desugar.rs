use libcosyc_diagnostic::{
    error::{
        IssueTracker,
        CompilerError
    },
    source::Span
};
use libcosyc_parse::syntax as ast;
use crate::ir;

/// Handles the conversion of the AST into IR.
pub struct ASTDesugar<'a> {
    issues : &'a mut IssueTracker,
    src : &'a str
}

impl<'a> ASTDesugar<'a> {
    /// Creates a new instance from this issue tracker and source file.
    pub fn new(issues : &'a mut IssueTracker, src : &'a str) -> Self {
        Self { issues, src }
    }

    /// Reports an error to the issue tracker.
    pub fn report<T>(&mut self, error : CompilerError) -> Option<T> {
        self.issues.report_error(error);
        None
    }

    /// Renders this span using the content from the source file.
    pub fn render(&self, span : &Span) -> &'a str {
        span.render(&self.src)
    }

    /// Generates the instructions for expressions.
    pub fn visit_expr(&mut self, term : ast::Term) -> Option<ir::Inst> {
        let span = term.span;
        let kind = match term.kind {
            ast::TermKind::Variable => unimplemented!(),
            ast::TermKind::Integral => ir::InstKind::Integral,
            ast::TermKind::Primitive => {
                match self.render(&span) {
                    _ => unimplemented!()
                }
            },
            ast::TermKind::TypeAnno { value, ty } => unimplemented!(),
            ast::TermKind::BinaryOp { kind, left, right } => unimplemented!(),
            ast::TermKind::UnaryOp { kind, value } => unimplemented!(),
            ast::TermKind::Call { intrinsic, callsite, params } => unimplemented!()
        };
        let datatype = ir::Type::Unknown;
        Some(ir::Inst{ span, datatype, kind })
    }
}
