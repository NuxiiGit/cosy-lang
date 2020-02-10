use cosyc::lexer::scanner::FileScanner;
use cosyc::diagnostics::IssueTracker;
use cosyc::lexer::scanner::CharKind;

fn main() {
    let mut scanner = FileScanner::open("examples/tests/bleh.cosy").unwrap();
    println!("begin");
    loop {
        let kind = scanner.skip();
        //println!("{:?} {:?}", kind, scanner.substr());
        if kind == CharKind::EoF {
            break;
        }
    }
    println!("end");

    /*let mut issues = IssueTracker::new();
    let mut lexer = Lexer::from(scanner, &mut issues);
    println!("{:?}", lexer.next());
    println!("{:?}", lexer.next());
    println!("{:?}", lexer.next());
    println!("{:?}", lexer.next());
    if issues.level().is_some() {
        for e in issues {
            println!("{}", e);
        }
    }*/
}

/*
use cosyc::lexer::scanner::Scanner;

fn main() {
    let src = "hello!ーあなた";
    let mut scanner = Scanner::new(src);
    println!("{:?}", scanner.chr());
    scanner.advance(); // h
    scanner.advance(); // e
    scanner.advance(); // l
    scanner.advance(); // l
    println!("{}", scanner.substr());
    let slice = scanner.span();
    scanner.clear();
    println!("{} {}", &src[slice.byte_begin..slice.byte_end], slice);
    scanner.advance(); // o
    scanner.advance(); // !
    scanner.advance(); // ー
    scanner.advance(); // あ
    scanner.advance(); // な
    scanner.advance(); // た
    println!("{}", scanner.substr());
    let slice = scanner.span();
    println!("{} {}", &src[slice.byte_begin..slice.byte_end], slice);
}
*/

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