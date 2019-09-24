mod runner;

use runner::parser::*;
use runner::lexer::*;
use runner::interpreter::*;
use runner::essentials::error::Error;
use std::time::Instant;

fn main() {
    let t = Instant::now();
    // compile
    if let Some(ast) = Lexer::new(r#"12"#).into_ast() {
        // display the syntax tree
        println!("\nTree:\n{:#?}\n", &ast);
        // display the result
        match Interpreter::new().execute(ast) {
            Ok(x) => println!("Result = {:?}", x),
            Err(e) => println!("Runtime Error:\n{}", e)
        }
    }
    // log errors
    if let Some(errors) = Error::log() {
        println!("\nErrors:");
        for e in errors {
            println!(" |> {}", e);
        }
    }
    // record time
    let micro = t.elapsed().as_micros();
    let second : f64 = (micro as f64) / 1000000.0;
    println!("\nCompile Time:\n{} s ({} Ms)", second, micro);
}