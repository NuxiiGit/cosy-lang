mod runner;

use runner::lexer;

fn main() {
    match lexer::lex("'exit if (_variable_name) { repeat { '{ this is also a comment }' \"this is a string\" 1234 5.678 } until (_variable_name2); } '' this is a comment") {
        Some(tokens) => {
            println!("Success!");
            for token in tokens {
                println!("{}", token);
            }
        },
        None => println!("Error!")
    }
}