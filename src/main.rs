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

use cosyc;

use std::time::Instant;

fn main() {
	let now = Instant::now();
	//let mut sess = Session::read("workspace/sandbox.cosy").unwrap();
	//let mut parser = Parser::from(&mut sess);
	//let ast = parser.parse_program();
	//println!("AST: {:#?}", ast);
	let dt = now.elapsed();
	println!("{} s / {} ms / {} Ms", dt.as_secs(), dt.as_millis(), dt.as_micros());
	//println!("{}", sess);
}
