use libcosyc_diagnostic::Session;
use libcosyc_parse as parse;

pub fn test() {
    let _src = "_yo waddup `>>=` __`that's what they call a 'monad' :)\n--_ 1st 1_+_2_=_3_wtf'";
    let mut sess = Session::load("examples/test.cosy");
    let ast = parse::build_ast(&sess.src as &str, &mut sess.issues);
    println!("{:?}", ast);
}
