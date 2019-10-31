mod runner;

use runner::compiler::*;
use runner::evaluator::*;

use std::time::Instant;

fn main() {
    let t = Instant::now();
    println!("\nCompiling...");
    // compile
    match Parser::from(Lexer::from(r#"123 + "2""#)).parse() {
        Ok(ast) => {
            // record time
            let dt = t.elapsed();
            println!("\nCompile Time:\n{} ms ({} Ms)", 
                    dt.as_millis(), dt.as_micros());
            // interpret
            println!("\nSyntax Tree:\n{:#?}", ast);
            match Interpreter::new().execute(ast) {
                Ok(value) => println!("\nInterpreter Result:\n{:?}", value),
                Err(e) => println!("\nRuntime Error! {}", e)
            }
        },
        Err(e) => println!("\nCompiler Error! {}", e)
    }
}