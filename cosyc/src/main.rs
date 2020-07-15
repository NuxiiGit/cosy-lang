use libcosyc_diagnostics::*;
use libcosyc_span::*;

fn main() {
    let mut sess = Session::from(format!("hello world"));
    sess.filepath = format!("some_location.cosy");
    let span = Span {
        begin : 2,
        end : 7
    };
    Diagnostic::from(&span)
            .reason(format!("just testing uwu"))
            .note(format!("alright"))
            .note(format!("but did you know that uhhhhh"))
            .level(ErrorLevel::Bug)
            .report(&mut sess);
    println!("{}", sess);
}
