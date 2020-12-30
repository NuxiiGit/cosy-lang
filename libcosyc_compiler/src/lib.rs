use libcosyc_diagnostic as diagnostic;
use diagnostic::{ Session, error::CompilerError, source::Span };

pub fn test() {
    let mut sess = diagnostic::Session::from("hello world !!!!".to_string());
    let span = Span::new(2, 10);
    sess.issues.report_error(CompilerError::new()
            .span(&span)
            .reason("something weird")
            .note("consider removing this")
            .note("really do consider"));
    println!("{}", sess);
}
