mod runner;

use runner::compiler::scanner::Tokeniser;
use runner::compiler::parser::Builder;

fn main() {
    match r#"d || a && b == c"#.tokenise().into_ast() {
        Ok(ast) => println!("Tree:\n{:#?}", ast),
        Err(e) => println!("{}", e)
    }
}