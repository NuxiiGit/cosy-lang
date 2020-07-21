use libcosyc_diagnostics::*;
use libcosyc_span::*;
use libcosyc_lexer::Lexer;

fn main() {
    let src = "a''_b''_1_+_3";
    let mut lexer = Lexer::from(src);
    println!("{:?}", lexer.generate_token());
    let span = lexer.span();
    println!("{:?}", &src[span.begin..span.end]);
    let mut sess = Session::from(format!("hello wo\nrld"));
    sess.filepath = format!("some_location.cosy");
    Diagnostic::from(&Span { begin : 2, end : 5 })
            .reason(format!("just testing uwu"))
            .note(format!("alright"))
            .note(format!("but did you know that uhhhhh"))
            .level(ErrorLevel::Bug)
            .report(&mut sess);
    Diagnostic::from(&Span { begin : 5, end : 9 })
            .reason(format!("another one"))
            .note(format!("bwehhh,,,"))
            .level(ErrorLevel::Fatal)
            .report(&mut sess);
    println!("{}", sess);
}
