use cosyc::{
    token::Token,
    scanner::{
        Lexer,
        StrScanner
    }
};

fn main() {
    for result in Lexer::lex(StrScanner::from(r#"
//| documentation comment
//| docs can be continued to new lines like so

// if not (unless) statement
unless condition {
    stuff();
} else if otherwise {
    other_stuff();
} else {
    more_stuff();
}
"#)) {
        match result {
            Ok(Token { kind, span }) => println!("{}: {:?}", span, kind),
            Err(e) => println!("{}", e)
        }
    }
}