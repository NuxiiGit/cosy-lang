mod tokeniser;

use tokeniser::{ CharClump, Tokeniser };

use libcosyc_diagnostics::{ Error, ErrorKind };

pub struct Lexer<'a> {
    tokeniser : Tokeniser<'a>,
}
impl<'a> Lexer<'a> {
    /// Creates a new lexer from this string slice.
    pub fn from(src : &'a str) -> Self {
        Self {
            tokeniser : Tokeniser::from(src)
        }
    }

    /// Creates a new error of this kind and reason.
    fn make_error(&self, reason : &'static str) -> Error<'a> {
        let token = self.make_token(TokenKind::Unknown);
        let kind = ErrorKind::Fatal;
        Error { reason, token, kind }
    }

    /// Creates a new token of this kind.
    fn make_token(&self, kind : TokenKind) -> Token<'a> {
        let context = self.reader.context();
        Token { context, kind }
    }
}
impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, Error<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        Some('search: loop {
            self.reader.reset_context();
            let next = self.reader.next();
            let peek = self.reader.peek();
            let result = match next {
                x if x.is_valid_whitespace() => {
                    while self.reader.peek()
                            .is_valid_whitespace() {
                        self.reader.next();
                    }
                    continue 'search;
                },
                CharKind::Minus if peek == CharKind::Minus => {
                    // line comments
                    while !self.reader.peek()
                            .is_valid_ending() {
                        self.reader.next();
                    }
                    continue 'search;
                },
                CharKind::LeftBrace if peek == CharKind::Minus => {
                    // block comments
                    self.reader.next();
                    let mut nests = 1;
                    loop {
                        let next = self.reader.next();
                        let peek = self.reader.peek();
                        match (next, peek) {
                            (_, CharKind::EoF) => {
                                break Err("unterminated block comment");
                            },
                            (CharKind::LeftBrace, CharKind::Minus) => {
                                self.reader.next();
                                nests += 1
                            },
                            (CharKind::Minus, CharKind::RightBrace) => {
                                self.reader.next();
                                if nests == 1 {
                                    continue 'search;
                                } else {
                                    nests -= 1;
                                }
                            },
                            _ => ()
                        }
                    }
                },
                x if x.is_valid_digit() => {
                    while self.reader.peek()
                            .is_valid_digit() {
                        self.reader.next();
                    }
                    Ok(TokenKind::Literal(LiteralKind::Integer))
                },
                x if x.is_valid_graphic() => {
                    while self.reader.peek()
                            .is_valid_graphic() {
                        self.reader.next();
                    }
                    Ok(match self.reader.substr() {
                        "var" => TokenKind::Keyword(KeywordKind::Var),
                        "if" => TokenKind::Keyword(KeywordKind::If),
                        "else" => TokenKind::Keyword(KeywordKind::Else),
                        _ => TokenKind::Identifier
                    })
                },
                x if x.is_valid_operator() => {
                    let kind = match x {
                        CharKind::Bar => OperatorKind::Bar,
                        CharKind::Caret => OperatorKind::Caret,
                        CharKind::Ampersand => OperatorKind::Ampersand,
                        CharKind::Bang => OperatorKind::Bang,
                        CharKind::Equals => OperatorKind::Equals,
                        CharKind::LessThan => OperatorKind::LessThan,
                        CharKind::GreaterThan => OperatorKind::GreaterThan,
                        CharKind::Plus => OperatorKind::Plus,
                        CharKind::Minus => OperatorKind::Minus,
                        CharKind::Asterisk => OperatorKind::Asterisk,
                        CharKind::ForwardSlash => OperatorKind::ForwardSlash,
                        CharKind::Percent => OperatorKind::Percent,
                        _ => OperatorKind::Other
                    };
                    while self.reader.peek()
                            .is_valid_operator() {
                        self.reader.next();
                    }
                    Ok(match self.reader.substr() {
                        _ => TokenKind::Operator(kind)
                    })
                },
                CharKind::LeftParen => Ok(TokenKind::Symbol(SymbolKind::LeftParen)),
                CharKind::RightParen => Ok(TokenKind::Symbol(SymbolKind::RightParen)),
                CharKind::LeftBrace => Ok(TokenKind::Symbol(SymbolKind::LeftBrace)),
                CharKind::RightBrace => Ok(TokenKind::Symbol(SymbolKind::RightBrace)),
                CharKind::SemiColon => Ok(TokenKind::Symbol(SymbolKind::SemiColon)),
                CharKind::Dollar => Ok(TokenKind::Symbol(SymbolKind::Dollar)),
                CharKind::Backtick => Ok(TokenKind::Symbol(SymbolKind::Backtick)),
                CharKind::Hashtag => {
                    if let CharKind::Graphic = self.reader.peek() {
                        self.reader.next();
                        while let CharKind::Graphic = self.reader.peek() {
                            self.reader.next();
                        }
                        Ok(TokenKind::Directive)
                    } else {
                        Err("expected graphic after hashtag symbol")
                    }
                },
                CharKind::Address => Ok(TokenKind::Symbol(SymbolKind::Address)),
                CharKind::EoF => Ok(TokenKind::EoF),
                _ => Err("unexpected symbol")
            };
            break 'search match result {
                Ok(kind) => Ok(self.make_token(kind)),
                Err(reason) => Err(self.make_error(reason))
            }
        })
    }
}