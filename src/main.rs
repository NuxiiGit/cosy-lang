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
    lexer.add("COMMENT", "''.*(\n|$)");
    if let Ok(tokens) = lexer.lex("if(->)'' test comment ()-> ()\n if ") {
        
    }
}
