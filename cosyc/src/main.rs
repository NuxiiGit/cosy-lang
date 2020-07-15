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
            .error_level(ErrorLevel::Bug)
            .report(&mut sess);
    println!("{}", sess);
}
