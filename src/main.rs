//use cosyc::common::diagnostics::IssueTracker;
//use cosyc::lexer::scanner::FileScanner;
//use cosyc::lexer::Lexer;

//use cosyc::session::Session;

use cosyc::parser::lexer::*;

use std::fs;
use std::time::Instant;

fn main() {
	let now = Instant::now();
	let mut lexer = Lexer::from("he->⸬-+---*::><=llo\r\n2\n3\r4world");
	let mut i = 0;
	loop {
		i += 1;
		let result = lexer.next();
		println!("{}.)", i);
		println!("  kind:    {:?}", result);
		println!("  context: {:?}", lexer.context());
		println!("  span:    {}\n", lexer.span());
		if let Ok(TokenKind::EoF) = result {
			break;
		}
	}

	//let sess = Session::read("examples/tests/bleh.cosy");
	/*let mut lexer = Lexer::from(&src);
	loop {
		match lexer.next() {
			Ok(token) if token.kind.is_eof() => break,
			_ => n += 1
			//Ok(token) => println!("{} ({:?})", token.context, token.kind),
			//Err(e) => println!("{}", e)
		}
	}*/
	let dt = now.elapsed();
	println!("{} s / {} ms / {} Ms", dt.as_secs(), dt.as_millis(), dt.as_micros());
	/*for error in sess.issues {
		println!("{}", error);
	}*/
}