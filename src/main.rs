mod runner;

use runner::compiler::scanner::Tokeniser;
use runner::compiler::parser::Builder;

fn main() {
    match r#"1 + 1 != f a > 3"#.tokenise().into_ast() {
        Ok(ast) => println!("Tree:\n{:#?}", ast),
        Err(e) => println!("{}", e)
    }
}