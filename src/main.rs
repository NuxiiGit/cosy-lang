//use cosyc::common::diagnostics::IssueTracker;
//use cosyc::lexer::scanner::FileScanner;
//use cosyc::lexer::Lexer;

//use cosyc::session::Session;

use cosyc::scanner::Scanner;

use std::fs;
use std::time::Instant;

fn main() {
	let now = Instant::now();
	let mut scanner = Scanner::from("hello\r\n\n\rworld");
	println!("{:?}", scanner.next()); // BOF
	println!("{:?}", scanner.next()); // h
	println!("{:?}", scanner.next()); // e
	println!("{:?}", scanner.next()); // l
	println!("{:?}", scanner.next()); // l
	println!("{:?}", scanner.next()); // o
	println!("{:?}", scanner.next()); // \r\n
	println!("{:?}", scanner.next()); // \n
	println!("{:?}", scanner.next()); // \r
	println!("{:?}", scanner.next()); // w
	println!("{:?}", scanner.next()); // o
	println!("{:?}", scanner.next()); // r
	println!("{:?}", scanner.next()); // l
	println!("{:?}", scanner.next()); // d
	println!("{:?}", scanner.next()); // EOF
	println!("{:?}", scanner.next()); // EOF
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