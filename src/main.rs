mod runner;

use runner::parser;
use runner::lexer::Lexer;
use runner::error::Error;
use std::time::Instant;

fn main() {
    let t = Instant::now();
    let mut lexer = Lexer::lex(r#"
            ' comment
            if condition == (-1 + 3) {
                '{
                    multi-line comment
                
                var k = "string";
            }"#);
    println!("Tokens:");
    while let Some(t) = lexer.next() {
        println!("{:?}", t.flavour());
    }
    if let Some(errors) = Error::log() {
        println!("\nErrors:");
        for e in errors {
            println!("{}", e);
        }
    }
    let micro = t.elapsed().as_micros();
    let second : f64 = (micro as f64) / 1000000.0;
    println!("\nTime: {} s ({} Ms)", second, micro);
}