//use cosyc::common::diagnostics::IssueTracker;
//use cosyc::lexer::scanner::FileScanner;
//use cosyc::lexer::Lexer;

//use cosyc::session::Session;

use cosyc::parser::lexer::*;

use std::fs;
use std::time::Instant;

fn main() {
	let now = Instant::now();
	let src = "_he_++_wwoo_~~__ uwu->⸬-+_hello---*::><=llo\r\n2\n3\r4world";
	let mut lexer = Lexer::from(src);
	let mut i = 0;
	loop {
		i += 1;
		let result = lexer.next();
		let span = lexer.span();
		println!("\n{}.)", i);
		println!("  kind:    {:?}", result);
		println!("  span:    {}", span);
		println!("  context: {:?}", span.render(src));
		if matches!(result, TokenKind::EoF) {
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