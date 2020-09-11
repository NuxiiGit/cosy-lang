use libcosyc_diagnostics::Session;
use libcosyc_concrete::parse::Parser;
use libcosyc_abstract::Desugar;

fn main() {
    let mut sess = Session::from(format!("(()+)"));
    let mut parser = Parser::from(&sess.src as &str);
    let cst = parser.parse_stmt();
    //println!("{:?}", cst);
    let _ast = cst.desugar(&mut sess.issues);
    println!("{}", sess);
}
