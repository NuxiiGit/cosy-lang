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
	let src = "((\nabc_+_coo\r(\n\r\n\r\r l)));";
	let mut sess = Session::from(String::from(src));
	let mut parser = Parser::from(&mut sess);
	let result = parser.parse_expr();
	match result {
		Ok(ast) => println!("{:#?}", ast),
		Err(e) => sess.issues.report(e)
	}
	let dt = now.elapsed();
	println!("{} s / {} ms / {} Ms", dt.as_secs(), dt.as_millis(), dt.as_micros());
	println!("{}", sess);
}