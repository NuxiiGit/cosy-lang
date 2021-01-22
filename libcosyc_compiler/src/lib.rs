use libcosyc_diagnostic::Session;
use libcosyc_parse as parse;
use libcosyc_ir as cosyir;
use libcosyc_codegen as codegen;

pub fn test() {
    let mut names = codegen::ident::NameTable::new();
    names.set("hi");
    names.set("hi");
    println!("{}", names.get("hi").unwrap());
    names.set("hi");
    println!("{}", names.get("hi").unwrap());
    names.unset("hi");
    names.set("hiya");
    println!("{}", names.get("hiya").unwrap());
    println!("{}", names.get("hi").unwrap());

    let mut sess = Session::load("examples/test.cosy");
    if let Some(ast) = parse::build_ast(&sess.src as &str, &mut sess.issues) {
        if let Some(ir) = cosyir::generate_ir(ast, &sess.src as &str, &mut sess.issues) {
            let mut out = String::new();
            if codegen::generate_c(ir, &sess.src as &str, &mut sess.issues, &mut out).is_some() {
                println!("{}", out);
            }
        }
    }
    println!("{}", sess);
}
