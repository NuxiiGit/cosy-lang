use libcosyc_diagnostic::{ Session, error::{ CompilerError, ErrorLevel } };
use libcosyc_scan::{ token::TokenKind, Lexer };

pub fn test() {
    let _src = "_yo waddup `>>=` __`that's what they call a 'monad' :)\n--_ 1st 1_+_2_=_3_wtf'";
    let mut sess = Session::load("examples/test.cosy");
    let mut lexer = Lexer::from(&sess.src as &str);
    let mut level = ErrorLevel::default();
    loop {
        let token = lexer.generate_token();
        sess.issues.report_error(CompilerError::new()
                .span(lexer.span())
                .level(level.clone())
                .reason("here is a token")
                .note("it really is a token"));
        if let TokenKind::EoF = token {
            break;
        }
        level = match level {
            ErrorLevel::Lint => ErrorLevel::Warning,
            ErrorLevel::Warning => ErrorLevel::Fatal,
            ErrorLevel::Fatal => ErrorLevel::Bug,
            ErrorLevel::Bug => ErrorLevel::Lint
        };
    }
    println!("{}", sess);
}
