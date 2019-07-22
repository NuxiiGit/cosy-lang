mod runner;

use runner::lexer;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    match lexer::lex(r#"'exit if (1st_variable_name) { repeat { '{ this is also a comment }' "this is a string" 1234 5.678 0.910 11. } until (2nd_variable_name); } '' this is a"#) {
        Some(tokens) => {
            println!("Success!");
            /*for token in tokens {
                println!("{}", token);
            }*/
        },
        None => println!("Error!")
    }
    let duration = now.elapsed().as_micros();
    let duration_s : f64 = (duration as f64) / 1000000.0;
    println!("Time: {} s ({} Ms)", duration_s, duration);
}