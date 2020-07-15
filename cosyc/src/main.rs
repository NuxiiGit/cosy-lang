use libcosyc_diagnostics::*;

fn main() {
    let mut sess = Session::from(format!("hello\nworld"));
    let span = Span {
        begin : 3,
        end : 6
    };
    Diagnostic::from(span)
            .reason(format!("just testing uwu"))
            .report(&mut sess);
    println!("{}", sess);
}
