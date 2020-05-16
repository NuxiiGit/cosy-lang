//use cosyc::common::diagnostics::IssueTracker;
//use cosyc::lexer::scanner::FileScanner;
//use cosyc::lexer::Lexer;

//use cosyc::session::Session;

use cosyc::parser::lexer::*;
//use cosyc::parser::*;
//use cosyc::issues::*;

use std::fs;
use std::time::Instant;

fn main() {
	let now = Instant::now();
	let src = "1;``--'\n{-(){} var-} {-a;";
	let mut lexer = Lexer::from(src);
	loop {
		let span = lexer.span();
		let token = lexer.token();
		if let TokenKind::EoF = token {
			break;
		}
		println!("token={:?}, span={:?}, str={:?}", token, span, &src[span.begin..span.end]);
		lexer.advance();
	}
	
	/*let lexer = Lexer::from(src);
	let mut issues = IssueTracker::new();
	let mut parser = Parser::new(&mut issues, lexer);
	if let Some(prog) = parser.parse_program() {
		println!("program:");
		println!("  {:?}", prog);
		println!("{:?}", &src[1..2]);
	}
	println!("\nerrors:");
	for e in issues {
		println!("  {}", e);
	}*/

	/*let mut i = 0;
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
	}*/
	let dt = now.elapsed();
	println!("{} s / {} ms / {} Ms", dt.as_secs(), dt.as_millis(), dt.as_micros());
	/*for error in sess.issues {
		println!("{}", error);
	}*/
}