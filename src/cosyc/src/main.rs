//use cosyc::common::diagnostics::IssueTracker;
//use cosyc::lexer::scanner::FileScanner;
//use cosyc::lexer::Lexer;

use libcosyc_lexer::Lexer;

use std::fs;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let src = fs::read_to_string("examples/tests/bleh.cosy")
            .expect("unable to read file");
    let mut lexer = Lexer::from(&src);
    let mut n = 0;
    loop {
        match lexer.next() {
            Ok(token) if token.kind.is_eof() => break,
            _ => n += 1
            //Ok(token) => println!("{} ({:?})", token.context, token.kind),
            //Err(e) => println!("{}", e)
        }
    }
    let dt = now.elapsed();
    println!("n={}", n);
    println!("{} s / {} ms / {} Ms", dt.as_secs(), dt.as_millis(), dt.as_micros());
}