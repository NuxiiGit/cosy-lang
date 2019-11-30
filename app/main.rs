use cosyc::{
    token::Token,
    scanner::{
        Lexer,
        StrScanner
    }
};

fn main() {
    for result in Lexer::lex(StrScanner::from(r#"
    123;
    if (`something is cool`) {}
    "bleh"
  alright5
     a'
    "#)) {
        match result {
            Ok(Token { kind, span }) => println!("{}: {:?}", span, kind),
            Err(e) => println!("{}", e)
        }
    }
}