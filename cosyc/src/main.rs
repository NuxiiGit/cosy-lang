use libcosyc_diagnostics::*;

fn main() {
    let mut sess = Session::from(format!("hello world"));
    let span = Span {
        begin : 2,
        end : 5
    };
    Diagnostic::from(span)
            .reason(format!("just testing uwu"))
            .report(&mut sess);
    println!("{}", sess);
}
