mod runner;

use runner::parser::Parser;
use runner::lexer::Lexer;
use runner::error::Error;
use std::time::Instant;

fn main() {
    let t = Instant::now();
    // compile
    let scanner = Lexer::new(r#"(1+3)*3"#);
    let parser = Parser::new(scanner);
    let ast = parser.into_ast();
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