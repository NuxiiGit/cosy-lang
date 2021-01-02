//use libcosyc_diagnostic as diagnostic;
use libcosyc_scan as scan;
use scan::{ token::TokenKind, Lexer };

pub fn test() {
    let src = "_yo waddup `>>=` __`that's what they call a 'monad' :)\n--_ 1st 1_+_2_=_3_wtf'";
    let mut lexer = Lexer::from(src);
    loop {
        let token = lexer.generate_token();
        println!("{:?} {}", token, lexer.span().render(src));
        if let TokenKind::EoF = token {
            break;
        }
    }
}
