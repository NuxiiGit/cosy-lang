pub mod scanner;

use crate::diagnostics::error::{ Error, Session };
use crate::syntax::token::*;

use scanner::Scanner;
use std::char;

/// An iterator over a string slice which produces `Token`s.
pub struct Lexer<'a, 'b> {
    sess : &'a mut Session,
    scanner : Scanner<'b>
}
impl<'a, 'b> Lexer<'a, 'b> {
    /// Creates a new lexer from this string scanner and parser session.
    pub fn from(sess : &'a mut Session, scanner : Scanner<'b>) -> Self {
        Lexer { sess, scanner }
    }

    /// Returns the next token.
    pub fn next(&mut self) -> Token {
        'search:
        loop {
            self.scanner.clear();
            let kind = if let Some(x) = self.scanner.advance() {
                let peek = self.scanner.chr();
                if valid_whitespace(&x) {
                    // trim whitespace
                    self.scanner.advance_while(valid_whitespace);
                    continue 'search;
                } else if x == '/' && peek == Some(&'/') {
                    // trim line comment
                    self.scanner.advance();
                    let documentation = Some(&'|') == self.scanner.chr();
                    self.scanner.advance_while(|x| *x != '\n');
                    if documentation {
                        TokenKind::Documentation
                    } else {
                        continue 'search;
                    }
                } else if x == '/' && peek == Some(&'*') {
                    // trim block comments
                    self.scanner.advance();
                    let mut nests = 1;
                    loop {
                        match self.scanner.advance() {
                            Some('*') if Some(&'/') == self.scanner.chr() => {
                                self.scanner.advance();
                                if nests == 1 {
                                    break;
                                } else {
                                    nests -= 1;
                                }
                            },
                            Some('/') if Some(&'*') == self.scanner.chr() => {
                                self.scanner.advance();
                                nests += 1
                            },
                            Some(_) => (),
                            None => {
                                self.error("unterminated block comment");
                                break;
                            }
                        }
                    }
                    continue 'search;
                } else if valid_digit(&x) {
                    // lex numbers
                    let mut is_real = false;
                    while let Some(x) = self.scanner.chr() {
                        if let '.' = x {
                            if is_real {
                                break;
                            } else {
                                is_real = true;
                            }
                        } else if !valid_digit(x) {
                            break;
                        }
                        self.scanner.advance();
                    }
                    TokenKind::Literal(if is_real {
                        LiteralKind::Real
                    } else {
                        LiteralKind::Integer
                    })
                } else if valid_graphic(&x) {
                    // lex keywords and identifiers
                    self.scanner.advance_while(valid_graphic);
                    match self.scanner.substr() {
                        "var" => TokenKind::Keyword(KeywordKind::Var),
                        "const" => TokenKind::Keyword(KeywordKind::Const),
                        "if" => TokenKind::Keyword(KeywordKind::If),
                        "unless" => TokenKind::Keyword(KeywordKind::Unless),
                        "else" => TokenKind::Keyword(KeywordKind::Else),
                        "then" => TokenKind::Keyword(KeywordKind::Then),
                        "switch" => TokenKind::Keyword(KeywordKind::Switch),
                        "case" => TokenKind::Keyword(KeywordKind::Case),
                        "is" => TokenKind::Keyword(KeywordKind::Is),
                        "while" => TokenKind::Keyword(KeywordKind::While),
                        "until" => TokenKind::Keyword(KeywordKind::Until),
                        "repeat" => TokenKind::Keyword(KeywordKind::Repeat),
                        "for" => TokenKind::Keyword(KeywordKind::For),
                        "in" => TokenKind::Keyword(KeywordKind::In),
                        "function" => TokenKind::Keyword(KeywordKind::Function),
                        "object" => TokenKind::Keyword(KeywordKind::Object),
                        "new" => TokenKind::Keyword(KeywordKind::New),
                        _ => TokenKind::Identifier
                    }
                } else if valid_operator(&x) {
                    // lex operators
                    let kind = OperatorKind::Custom;
                    self.scanner.advance_while(valid_operator);
                    match self.scanner.substr() {
                        "->" => TokenKind::Symbol(SymbolKind::Arrow),
                        "=" => TokenKind::Symbol(SymbolKind::Assign),
                        "\\" => TokenKind::Symbol(SymbolKind::Backslash),
                        _ => TokenKind::Operator(kind)
                    }
                } else {
                    self.error("unicode characters are unsupported");
                    continue 'search;
                }
            } else {
                TokenKind::EoF
            };
            break self.tokenise(kind);
        }
    }

    /// Reports an error of this kind.
    pub fn error(&mut self, reason : &'static str) {
        let span = self.scanner.span();
        let token = Token {
            kind : TokenKind::Unknown,
            span
        };
        self.sess.report(Error { reason, token });
    }

    /// Creates a token of this kind from the scanner.
    pub fn tokenise(&mut self, kind : TokenKind) -> Token {
        let span = self.scanner.span();
        Token { kind, span }
    }
}

/// Returns whether this character is a valid operator character.
pub fn valid_operator(x : &char) -> bool {
    if let '!' | '?' | '\'' | '_' |
            '@' | '&' |
            '+' | '-' | '*' | '/' | '\\' | '%' | '^' |
            '<' | '=' | '>' |
            '|' | '~' = x {
        true
    } else {
        false
    }
}

/// Returns whether this character is a valid whitespace character.
pub fn valid_whitespace(x : &char) -> bool {
    x.is_ascii_whitespace()
}

/// Returns whether this character is a valid identifier character.
pub fn valid_graphic(x : &char) -> bool {
    *x == '\'' || *x == '_' || x.is_ascii_alphanumeric()
}

/// Returns whether this character is a valid number character.
pub fn valid_digit(x : &char) -> bool {
    x.is_ascii_digit()
}