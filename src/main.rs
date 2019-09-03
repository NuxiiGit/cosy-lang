mod runner;

use runner::scanner::*;
use runner::lexer::*;

//use runner::parser;
//use std::time::Instant;

fn main() {
    match lex(r#"
    if condition==(-1+3){
        var k="string";
    }
    "#) {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token.flavour());
            }
        },
        Err((msg, row, col)) => {
            println!("Error! {} at row={} col={}", msg, row, col);
        }
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
