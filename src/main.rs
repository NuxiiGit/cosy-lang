mod runner;

use runner::parser::Parser;
use runner::lexer::Lexer;
use runner::error::Error;
use std::time::Instant;

fn main() {
    let t = Instant::now();
    let ast = Parser::parse(r#"1 + 3 - 4"#);
    if let Some(errors) = Error::log() {
        println!("\nErrors:");
        for e in errors {
            println!(" |> {}", e);
        }
    }
    let micro = t.elapsed().as_micros();
    let second : f64 = (micro as f64) / 1000000.0;
    println!("\nTime:\n{} s ({} Ms)", second, micro);
}