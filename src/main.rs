mod runner;

use runner::compiler::scanner::Tokeniser;

fn main() {
    for result in "1 `plus` 4 {} if".tokenise() {
        match result {
            Ok(token) => println!("{}", token),
            Err(errors) => {
                for e in errors {
                    println!("{}", e);
                }
            },
        }
    }
}