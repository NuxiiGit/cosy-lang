mod runner;

use runner::lexer::Lexer;

fn main() {
    let mut lexer : Lexer = Lexer::new();
    lexer.add("LBRACE", r"\(");
    lexer.add("RBRACE", r"\)");
    lexer.add("IF", "if");
    lexer.add("ARROW", "->");
    lexer.add("MINUS", "-");
    println!("{}", lexer);
}
