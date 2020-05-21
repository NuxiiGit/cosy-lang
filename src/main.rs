use cosyc::{
	parse::{
		Parser, Node,
		lex::{ Lexer, TokenKind }
	},
	common::{
		Session,
		diagnostics::{ IssueTracker, error::Error }
	}
};

use std::time::Instant;

fn main() {
	let now = Instant::now();
	let src = "((( uwu )));";
	let mut sess = Session::from(String::from(src));
	let mut parser = Parser::from(&mut sess);
	let result = parser.parse_stmt();
	match result {
		Ok(Node { span, content }) => println!("{}\n{:#?}", span, content),
		Err(e) => println!("{}", e)
	}
	let dt = now.elapsed();
	println!("{} s / {} ms / {} Ms", dt.as_secs(), dt.as_millis(), dt.as_micros());
	for error in &sess.issues {
		println!("{}", error);
	}
}