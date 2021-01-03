use libcosyc_diagnostic::Session;
use libcosyc_parse as parse;

pub fn test() {
    let mut sess = Session::load("examples/test.cosy");
    if let Some(ast) = parse::build_ast(&sess.src as &str, &mut sess.issues) {
        println!("{}", parse::render::render_as_lisp(&sess.src, &ast));
    }
    println!("{}", sess);
}
