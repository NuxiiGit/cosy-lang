use libcosyc_source::Span;
use libcosyc_diagnostics::{ Diagnostic, Session, ErrorLevel };
use libcosyc_concrete::Parser;

fn main() {
    let src = "1 1_st";
    let mut parser = Parser::from(src);
    let syntax = parser.parse_expr_terminal();
    println!("{:?}", syntax);
    let mut sess = Session::from(format!("hello wo\n\n\nrld\n\n\nhihihihihihihihihih"));
    sess.filepath = format!("some_location.cosy");
    Diagnostic::from(&Span { begin : 2, end : 5 })
            .reason(format!("just testing uwu"))
            .note(format!("alright"))
            .note(format!("but did you know that uhhhhh"))
            .level(ErrorLevel::Bug)
            .report(&mut sess.issues);
    Diagnostic::from(&Span { begin : 5, end : 15 })
            .reason(format!("another one"))
            .note(format!("bwehhh,,,"))
            .level(ErrorLevel::Fatal)
            .report(&mut sess.issues);
    println!("{}", sess);
}
