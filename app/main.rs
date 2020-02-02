use cosyc::lexer::*;
use cosyc::diagnostics::error::Session;

fn main() {
    let src = "hello!ーあなた";
    let scanner = scanner::Scanner::new(src);
    let mut sess = Session::new();
    let mut lexer = Lexer::from(&mut sess, scanner);
    println!("{:?}", lexer.next());
    for error in &*sess {
        println!("{}", error);
    }
}

/* use cosyc::{
    lexer::*,
    parser::*
};

use std::fs;
use std::io::{
    Read,
    Write
};

fn main() {
    let inp = "tests/test.cosy";
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
    let scanner = StringScanner::from(&source);
    let lexer = Lexer::from(scanner);
    let result = Parser::parse(lexer);
    let s = match result {
        Ok(ast) => format!("{}", ast.to_string()),
        Err(es) => {
            es.iter().fold(String::from("Errors:"), |mut acc, e| {
                if !acc.is_empty() {
                    acc.push('\n');
                }
                acc.push_str(&e.to_string());
                acc
            })
        }
    };
    out.write(s.as_bytes())
            .expect("unable to write to file");
}*/