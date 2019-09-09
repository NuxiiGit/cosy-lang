mod runner;

use runner::parser::*;
use runner::lexer::Lexer;
use runner::essentials::error::Error;
use std::time::Instant;

fn main() {
    let t = Instant::now();
    // compile
    let ast = Lexer::new(r#"1 test testing another + 2"#)
            .into_ast();
    // record time
    let micro = t.elapsed().as_micros();
    let second : f64 = (micro as f64) / 1000000.0;
    println!("\nCompile Time:\n{} s ({} Ms)", second, micro);
    // display the syntax tree
    if let Some(program) = ast {
        println!("\nTree:\n{:#?}", program);
    }
    // log errors
    if let Some(errors) = Error::log() {
        println!("\nErrors:");
        for e in errors {
            println!(" |> {}", e);
        }
    }
}