use libcosyc_diagnostic::Session;
use libcosyc_parse as parse;

pub fn test() {
    let mut sess = Session::load("examples/test.cosy");
    let ast = parse::build_ast(&sess.src as &str, &mut sess.issues);
    println!("{}", sess);
    println!("{:#?}", ast);
}
