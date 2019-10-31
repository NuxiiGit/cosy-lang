mod runner;

use runner::compiler::*;
//use runner::evaluator::interpreter::Interpreter;

use std::time::Instant;

fn main() {
    let t = Instant::now();
    println!("\nCompiling...");
    // compile
    match Parser::from(Lexer::from(r#"1 ! 2 £ 3 $ 4 % 5 & 6 # 7 ~ 8 @ 9 . 10"#)).parse() {
        Ok(ast) => {
            // record time
            let dt = t.elapsed();
            println!("\nCompile Time:\n{} ms ({} Ms)", 
                    dt.as_millis(), dt.as_micros());
            // interpret
            println!("\nSyntax Tree:\n{:#?}", ast);
            /*match Interpreter::interpret(ast) {
                Ok(value) => println!("\nInterpreter Result:\n{:?}", value),
                Err(e) => println!("\n{}", e)
            }*/
        },
        Err(e) => println!("\n{}", e)
    }
}