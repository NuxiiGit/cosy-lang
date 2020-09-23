use libcosyc_diagnostics::Session;
use libcosyc_concrete::parse::Parser;
use libcosyc_abstract::Desugar;
use libcosyc_codegen::clang::Codegen;

fn main() {
    let mut sess = Session::from("12".to_string());
    let mut parser = Parser::from(&sess.src as &str);
    let cst = parser.parse_expr();
    //println!("{:?}", cst);
    if let Some(ast) = Desugar::from(&mut sess).desugar_expr(cst) {
        let mut out = String::new();
        let mut codegen = Codegen::new(&mut out);
        let _ = codegen.emit_expr(ast);
        println!("{}", out);
    }
    println!("{}", sess);
}
