mod runner;

use runner::compiler::scanner::Tokeniser;
use runner::compiler::parser::Builder;
use runner::evaluator::interpreter::Interpreter;

fn main() {
    match r#"255"#.tokenise().into_ast() {
        Ok(ast) => {
            println!("Tree:\n{:#?}", ast);
            match Interpreter::interpret(ast) {
                Ok(value) => println!("Value: {:?}", value),
                Err(e) => println!("{}", e)
            }
        },
        Err(e) => println!("{}", e)
    }
}