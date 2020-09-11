use libcosyc_diagnostics::Session;
use libcosyc_concrete::parse::Parser;
use libcosyc_abstract::Desugar;

fn main() {
    let mut sess = Session::from(format!("(+)"));
    let mut parser = Parser::from(&sess.src as &str);
    let _ast = parser.parse_stmt().desugar(&mut sess.issues);
    //println!("{:?}", ast);
    println!("{}", sess);
}
