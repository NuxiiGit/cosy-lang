use uwooc::source_pos::Span;
use uwooc::token::Token;
use uwooc::scanner::{
    Lexer,
    StrScanner
};

fn main() {
    for result in Lexer::lex(StrScanner::from(r#"123 alright5 a"#)) {
        match result {
            Ok(Token { kind, span }) => println!("{}: {:?}", span, kind),
            Err(e) => println!("{}", e)
        }
    }
}