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
    parse::{
        lex::{ Lexer },
        Parser
    },
    common::Session
};

use std::time::Instant;

fn main() {
    let now = Instant::now();
    let mut sess = Session::read("sandbox/sandbox.cosy").unwrap();
    let lexer = Lexer::from(&sess.src);
    let mut parser = Parser::from(lexer);
    println!("{:?}", parser.parse_expr_terminal());
    //let ast = parser.parse_program();
    //println!("AST: {:#?}", ast);
    let dt = now.elapsed();
    println!("{} s / {} ms / {} Ms", dt.as_secs(), dt.as_millis(), dt.as_micros());
    //println!("{}", sess);
}
