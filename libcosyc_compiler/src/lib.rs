use libcosyc_diagnostic::Session;
use libcosyc_scan::{ token::TokenKind, Lexer };

pub fn test() {
    let _src = "_yo waddup `>>=` __`that's what they call a 'monad' :)\n--_ 1st 1_+_2_=_3_wtf'";
    let mut sess = Session::from("examples/test.cosyc");
    let mut lexer = Lexer::from(&sess.src as &str);
    println!("{}", sess);
    loop {
        let token = lexer.generate_token();
        println!("{:?} {}", token, lexer.span().render(&sess.src));
        if let TokenKind::EoF = token {
            break;
        }
    }
}
