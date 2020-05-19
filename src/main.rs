use cosyc::parse::lexer::{ Lexer, TokenKind };

use std::fs;
use std::time::Instant;

fn main() {
	let now = Instant::now();
	let src = "xXXi_ -------->wud_nvrst\nøp_ÜXXx";
	let mut lexer = Lexer::from(src);
	loop {
		let token = lexer.advance();
		let span = lexer.span();
		println!("{:?}:\n  span: {}\n  str:{:?}", token, span, &src[span.begin..span.end]);
		if let TokenKind::EoF = token {
			break;
		}
	}
	let dt = now.elapsed();
	println!("{} s / {} ms / {} Ms", dt.as_secs(), dt.as_millis(), dt.as_micros());
	/*for error in sess.issues {
		println!("{}", error);
	}*/
}