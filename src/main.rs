use cosyc::{
	parse::{
		Parser,
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
	let src = "xXXi_ var bweh = 123;.22-------->wud_nvrst-------->\nøp_ÜXXx";
	let mut sess = Session::from(String::from(src));
	let mut parser = Parser::from(&mut sess);
	loop {
		let token = parser.next_token();
		if let TokenKind::EoF = token {
			break;
		}
	}
	let dt = now.elapsed();
	println!("{} s / {} ms / {} Ms", dt.as_secs(), dt.as_millis(), dt.as_micros());
	for error in &sess.issues {
		println!("{}", error);
	}
}