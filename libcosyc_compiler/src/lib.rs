use libcosyc_diagnostic::Session;
use libcosyc_parse as parse;
use libcosyc_ir as cosyir;
use libcosyc_codegen as codegen;

pub fn test() {
    let mut sess = Session::load("examples/test.cosy");
    if let Some(ast) = parse::build_ast(&sess.src as &str, &mut sess.issues) {
        if let Some(ir) = cosyir::generate_ir(ast, &sess.src as &str, &mut sess.issues) {
            let mut out = String::new();
            if codegen::generate_c(ir, &sess.src as &str, &mut sess.issues, &mut out).is_some() {
                println!("{}", out);
            }
        }
    }
    if sess.errors_occurred() {
        println!("{}", sess);
    }
}
