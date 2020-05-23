use cosyc::{
	parse::{
		Parser,
		lex::{ Lexer, TokenKind }
	},
	common::{
		Session,
		diagnostics::{ IssueTracker }
	}
};

use std::time::Instant;

fn main() {
	let now = Instant::now();
	let src = "(((2abc_+_cool)));";
	let mut sess = Session::from(String::from(src));
	let mut parser = Parser::from(&mut sess);
	let result = parser.parse_expr();
	match result {
		Ok(ast) => println!("{:#?}", ast),
		Err(e) => println!("{}", e)
	}
	let dt = now.elapsed();
	println!("{} s / {} ms / {} Ms", dt.as_secs(), dt.as_millis(), dt.as_micros());
	for issue in &sess.issues {
		println!("{}", issue);
	}
}