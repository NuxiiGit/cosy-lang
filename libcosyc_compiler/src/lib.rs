use libcosyc_diagnostic::Session;
use libcosyc_parse as parse;
use libcosyc_ir as cosyir;
use libcosyc_codegen::llvm as codegen;

/// Starts a new compiler session using this file path.
pub fn open(path : &str) -> Session {
    Session::load(path)
}

/// Compiles this program to the desired level.
pub fn compile(sess : &mut Session) -> Option<()> {
    let ast = parse::build_ast(&sess.src, &mut sess.issues)?;
    let ir = cosyir::generate_ir(ast, &sess.src, &mut sess.issues)?;
    codegen::compile_ir(ir, &sess.src, &mut sess.issues)?;
    Some(())
}

pub fn test() {
    let mut sess = open("examples/test.cosy");
    compile(&mut sess);
    if sess.errors_occurred() {
        println!("{}", sess);
    }
}
