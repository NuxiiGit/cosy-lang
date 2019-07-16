mod runner;

use runner::lexer::Lexer;
use runner::token::Token;

fn main() {
    let mut lexer : Lexer = Lexer::new();
    lexer.ignore(r"\s+");
    lexer.add("LBRACE", r"\(");
    lexer.add("RBRACE", r"\)");
    lexer.add("IF", "if");
    lexer.add("MINUS", "-");
    lexer.add("ARROW", "->");
    lexer.add("COMMENT", r"(?:''.*(?:\n|$))|(?:'\{.*\}')");
    match lexer.lex("if(->)'{ test '' comment ()}'-> ()\n if ") {
        Ok(tokens) => {
            for token in &tokens {
                println!("({}, {})", token.ident, token.value);
            }
        },
        Err(msg) => println!("{}", msg)
    }
}