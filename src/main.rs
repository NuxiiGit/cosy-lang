mod runner;

use runner::lexer;
use runner::parser;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    match lexer::lex(r#"--(-1 + 4*(3)/8 - 1/2)"#) {
        Ok(tokens) => {
            println!("Success!");
            for token in &tokens {
                println!("{}", token);
            }
            match parser::parse(&tokens) {
                Ok(expr) => {
                    println!("{}", expr);
                },
                Err(msg) => {
                    println!("{}", msg);
                }
            }
        },
        Err(msg) => println!("Error! {}", msg)
    }
    let duration = now.elapsed().as_micros();
    let duration_s : f64 = (duration as f64) / 1000000.0;
    println!("Time: {} s ({} Ms)", duration_s, duration);
}