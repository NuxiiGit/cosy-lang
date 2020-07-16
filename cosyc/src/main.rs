use libcosyc_diagnostics::*;
use libcosyc_span::*;

fn main() {
    let mut sess = Session::from(format!("he\nllo\nwo\nrld"));
    sess.filepath = format!("some_location.cosy");
    Diagnostic::from(&Span { begin : 2, end : 7 })
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
