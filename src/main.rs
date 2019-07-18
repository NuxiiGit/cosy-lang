mod runner;

use runner::lexer::Lexer;

fn main() {
    let mut lexer : Lexer = Lexer::new();
    lexer.ignore(Some(|chars| {
        let whitespace : String = chars.take_while(|&x|
                x == ' ' || x == '\n' || x == '\t' || x == '\r').collect();
        if whitespace == "" {
            None
        } else {
            Some(whitespace)
        }
    }));
    match lexer.lex("x \n expression: &str") {
        Some(_) => println!("Success"),
        _ => println!("Failure")
    }
}