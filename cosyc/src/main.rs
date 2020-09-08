use libcosyc_source::Span;
use libcosyc_diagnostics::{ Diagnostic, Session, ErrorLevel };
use libcosyc_concrete::Parser;
use libcosyc_abstract::Desugar;

fn main() {
    let mut sess = Session::from(format!("18832'22"));
    let mut parser = Parser::from(&sess.src as &str);
    let parse_tree = parser.parse_expr_terminal();
    let ast = parse_tree.desugar(&mut sess.issues);
    println!("{:?}", ast);
    println!("{}", sess);

    /*let mut sess = Session::from(format!("hello wo\n\n\nrld\n\n\nhihihihihihihihihih"));
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
    */
}
