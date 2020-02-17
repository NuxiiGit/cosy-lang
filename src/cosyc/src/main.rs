//use cosyc::common::diagnostics::IssueTracker;
//use cosyc::lexer::scanner::FileScanner;
//use cosyc::lexer::Lexer;

use libcosyc_lexer::{ Lexer, scanner::Scanner };
use libcosyc_diagnostics::IssueTracker;

use std::time::Instant;

fn main() {
    let now = Instant::now();
    let scanner = Scanner::open("examples/tests/bleh.cosy")
            .expect("unable to load file for reading");
    let mut issues = IssueTracker::new();
    let lexer = Lexer::new(scanner, &mut issues);
    let tokens : Vec<_> = lexer.into();
    for token in tokens {
        println!("{} {:?} ({:?})", token.context, token.kind, token.context.src);
    }
    if issues.level().is_some() {
        for e in issues {
            println!("{}", e);
        }
    }
    let dt = now.elapsed();
    println!("{} s / {} ms / {} Ms", dt.as_secs(), dt.as_millis(), dt.as_micros());
}