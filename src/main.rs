#[macro_use]
pub mod runner;
pub mod language;

use runner::lexer::Lexer;

fn main() {
    let lexer : Lexer = language::generate_lexer();
    match lexer.lex("
            'exit
            if (1st_variable_name) {
                repeat {
                    '' this is a comment
                    '{
                        this is also
                        a comment
                    }'
                    \"this is a string\"
                    1234
                    5.678
                    .910
                    11.
                } until (2nd_variable_name);
            }") {
        Some(tokens) => {
            for token in &tokens {
                println!("{}", token);
            }
        },
        _ => println!("Failure")
    }
}