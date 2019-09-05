mod runner;

use runner::scanner::*;
use runner::lexer::*;
use runner::lexer_new::*;
use runner::error::*;

//use runner::parser;
//use std::time::Instant;

fn main() {
    let mut errors : Vec<Error> = Vec::new();
    println!("Tokens:");
    for t in Lexer::new(r#"
    '{
    if condition==(-1+3){
        var k="string";
    }
    "#, &mut |e| errors.push(e)) {
        println!("{:?}", t.flavour());
    }
    println!("\nErrors:");
    for e in &errors {
        println!("{}", e);
    }

    /*let now = Instant::now();
    let tokens = lexer::lex(r#"-(-1 + 4*(3)/8 - 1/2)"#);
    for token in &tokens {
        println!("{}", token);
    }
    /*match parser::parse(&tokens) {
        Ok(expr) => {
            println!("{}", expr);
        },
        Err(msg) => {
            println!("{}", msg);
        }
    }*/
    let duration = now.elapsed().as_micros();
    let duration_s : f64 = (duration as f64) / 1000000.0;
    println!("Time: {} s ({} Ms)", duration_s, duration);*/
}