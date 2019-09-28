mod runner;

//use runner::parser::*;
//use runner::lexer::*;
use runner::lexer_improved::*;
use runner::parser_improved::*;
use runner::interpreter::*;
use runner::collections::error::Error;
use std::time::Instant;

fn main() {
    let t = Instant::now();
    // compile
    match Parser::new(Lexer::new(r#"1 + 2 *3"#)).parse() {
        Ok(ast) => {
            // display the syntax tree
            println!("\nTree:\n{:#?}\n", ast);
            // display the result
            match Interpreter::new().execute(ast) {
                Ok(x) => println!("Result = {:?}", x),
                Err(e) => println!("Runtime Error:\n{}", e)
            }
        },
        Err(errors) => {
            // log errors
            println!("\nCompile Errors:");
            for e in errors {
                println!(" |> {}", e);
            }
        }
    }
    // record time
    let micro = t.elapsed().as_micros();
    let second : f64 = (micro as f64) / 1000000.0;
    println!("\nCompile Time:\n{} s ({} Ms)", second, micro);
}