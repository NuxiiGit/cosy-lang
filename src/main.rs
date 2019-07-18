mod runner;

use runner::lexer::Lexer;

fn main() {
    let mut lexer : Lexer = Lexer::new();
    lexer.ignore(Some(lex_whitespace!()));
    lexer.add("IF", lex_keyword!("if"));
    lexer.add("IFNOT", lex_keyword!("ifnot"));
    lexer.add("COMMENT", lex_line!("''"));
    lexer.add("COMMENT_2", lex_region!("'{", "}'"));
    match lexer.lex("   if '' '{ ifnot ifnot}'if end
    if '{another
    comment}'") {
        Some(tokens) => {
            for token in &tokens {
                println!("{}", token);
            }
        },
        _ => println!("Failure")
    }
}