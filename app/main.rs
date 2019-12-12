use cosyc::{
    syntax::token::Token,
    scanner::{
        Lexer,
        StrScanner
    }
};

use std::fs;
use std::io::{
    Read,
    Write
};

fn main() {
    let inp = "examples/members.cosy";
    let mut inp = fs::OpenOptions::new()
            .read(true)
            .open(inp)
            .expect("unable to open file for reading");
    let out = "temp/log.txt";
    let _ = fs::remove_file(out);
    let mut out = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(out)
            .expect("unable to open file for writing");
    let mut source = String::new();
    inp.read_to_string(&mut source)
            .expect("unable to read from file");
    for result in Lexer::lex(StrScanner::from(&source)) {
        let s = match result {
            Ok(Token { kind, span }) => format!("Token! {}: {:?}\n", span, kind),
            Err(e) => format!("Error! {}\n", e)
        };
        out.write(s.as_bytes())
                .expect("unable to write to file");
    }
}