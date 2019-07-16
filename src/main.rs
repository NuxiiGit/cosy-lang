mod runner;

use runner::lexer::Lexer;
use runner::token::Token;

fn main() {
    let mut lexer : Lexer = Lexer::new();
    // whitespace
    lexer.ignore(r"\s+");
    // comments
    lexer.add("COMMENT", r"(?:''.*(?:\n|$))|(?:'\{(?s:.)*\}')");
    // braces
    lexer.add("BRACKET_BEGIN", r"\(");
    lexer.add("BRACKET_END", r"\)");
    lexer.add("BLOCK_BEGIN", r"\{");
    lexer.add("BLOCK_END", r"\}");
    // label
    lexer.add("LABEL", r"'[a-zA-Z]+[a-zA-Z0-9]*");
    // conditions
    lexer.add("IF", "if");
    // lex and parse
    let source_code : &str = "
            'label3
            if () {
                '' this is a comment
                '{
                    test
                }'
            }";
    match lexer.lex(source_code) {
        Ok(tokens) => {
            for token in &tokens {
                println!("{}", token);
            }
        },
        Err(msg) => println!("{}", msg)
    }
}