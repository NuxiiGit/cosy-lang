#[macro_use]
pub mod runner;
pub mod language;

use runner::lexer::Lexer;

fn main() {
    let lexer : Lexer = language::generate_lexer();
    match lexer.lex("if 12.3") {
        Some(tokens) => {
            for token in &tokens {
                println!("{}", token);
            }
        },
        _ => println!("Failure")
    }
}