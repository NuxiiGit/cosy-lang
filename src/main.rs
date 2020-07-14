/*use cosyc::{
    parse::{
        Parser,
        lex::{ Lexer, TokenKind }
    },
    common::{
        Session,
        diagnostics::{ IssueTracker }
    }
};*/

use cosyc::{
    parse::lex::{ Lexer },
    common::Session
};

use std::time::Instant;

fn main() {
    let now = Instant::now();
    let mut sess = Session::read("sandbox/sandbox.cosy").unwrap();
    let mut lexer = Lexer::from(&sess.src);
    //let mut parser = Parser::from(&mut sess);
    //let ast = parser.parse_program();
    //println!("AST: {:#?}", ast);
    loop {
        let token = lexer.advance();
        println!("{:?}", token);
        if token.is_eof() {
            break;
        }
    }
    let dt = now.elapsed();
    println!("{} s / {} ms / {} Ms", dt.as_secs(), dt.as_millis(), dt.as_micros());
    //println!("{}", sess);
}
