mod runner;

use runner::lexer;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    match lexer::lex(r#"
        if condition {
            '' do stuff
            something("string");
        }
    "#) {
        Some(tokens) => {
            println!("Success!");
            for token in tokens {
                println!("{}", token);
            }
        },
        None => println!("Error!")
    }
    let duration = now.elapsed().as_micros();
    let duration_s : f64 = (duration as f64) / 1000000.0;
    println!("Time: {} s ({} Ms)", duration_s, duration);
}