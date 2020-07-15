use libcosyc_diagnostics::*;

fn main() {
    let mut sess = Session::from(format!("hello world"));
    sess.filepath = format!("some_location.cosy");
    let span = Span {
        begin : 2,
        end : 7
    };
    span.make_diagnostic()
            .reason(format!("just testing uwu"))
            .note(format!("alright"))
            .note(format!("but did you know that uhhhhh"))
            .level(ErrorLevel::Bug)
            .report(&mut sess);
    println!("{}", sess);
}
