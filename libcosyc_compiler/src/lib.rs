use libcosyc_diagnostic::Session;
use libcosyc_parse as parse;
use libcosyc_ir as cosyir;
use libcosyc_codegen as codegen;

/// Represents the output stream, in this case just a string.
type Out = String;

/// Starts a new compiler session using this file path.
pub fn open(path : &str) -> Session {
    Session::load(path)
}

/// Compiles this program to the desired level.
pub fn compile(sess : &mut Session) -> Option<Out> {
    let ast = parse::build_ast(&sess.src, &mut sess.issues)?;
    let ir = cosyir::generate_ir(ast, &sess.src, &mut sess.issues)?;
    let mut out = String::new();
    codegen::generate_c(ir, &sess.src, &mut sess.issues, &mut out)?;
    Some(out)
}

pub fn test() {
    let mut sess = open("examples/test.cosy");
    if let Some(out) = compile(&mut sess) {
        println!("{}", out);
    }
    if sess.errors_occurred() {
        println!("{}", sess);
    }
}
