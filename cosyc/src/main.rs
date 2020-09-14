use libcosyc_diagnostics::Session;
use libcosyc_concrete::parse::Parser;
use libcosyc_abstract::Desugar;

fn main() {
    let mut sess = Session::from("12".to_string());
    let mut parser = Parser::from(&sess.src as &str);
    let cst = parser.parse_expr();
    //println!("{:?}", cst);
    let _ast = Desugar::from(&mut sess.issues).desugar_expr(cst);
    println!("{}", sess);
}
