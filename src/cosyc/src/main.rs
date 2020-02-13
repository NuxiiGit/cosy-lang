//use cosyc::common::diagnostics::IssueTracker;
//use cosyc::lexer::scanner::FileScanner;
//use cosyc::lexer::Lexer;

use libcosyc_lexer::{ Lexer, scanner::FileScanner };
use libcosyc_diagnostics::IssueTracker;

fn main() {
    let scanner = FileScanner::open("examples/tests/bleh.cosy").unwrap();
    let mut issues = IssueTracker::new();
    let lexer = Lexer::from(scanner, &mut issues);
    let tokens : Vec<_> = lexer.into();
    for token in tokens {
        println!("{} {:?} ({:?})", token.context, token.kind, token.context.src);
    }
    if issues.level().is_some() {
        for e in issues {
            println!("{}", e);
        }
    }
}