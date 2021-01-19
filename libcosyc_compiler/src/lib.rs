use libcosyc_diagnostic::Session;
use libcosyc_parse as parse;
use libcosyc_ir as cosyir;

pub fn test() {
    let mut sess = Session::load("examples/test.cosy");
    if let Some(ast) = parse::build_ast(&sess.src as &str, &mut sess.issues) {
        if let Some(ir) = cosyir::desugar::desugar_ast(ast, &sess.src as &str, &mut sess.issues) {
            println!("{:?}", ir);
        }
    }
    println!("{}", sess);
}
