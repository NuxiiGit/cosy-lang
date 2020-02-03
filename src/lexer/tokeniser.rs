
use crate::diagnostics::error::{ Error, Session };
use crate::syntax::token::*;

use super::scanner::Cursor;

use std::char;

/// An iterator over a string slice which produces `Token`s.
pub struct Tokeniser<'a, 'b> {
    sess : &'a mut Session,
    scanner : Cursor<'b>
}
impl<'a, 'b> Tokeniser<'a, 'b> {
    /// Creates a new tokeniser from this string scanner and parser session.
    pub fn from(sess : &'a mut Session, scanner : Cursor<'b>) -> Self {
        Tokeniser { sess, scanner }
    }

    /// Returns the next token.
    pub fn next(&mut self) -> Token {
        'search:
        loop {
            self.scanner.clear();
            let kind = if let Some(x) = self.scanner.advance() {
                let peek = self.scanner.chr();
                if x.is_whitespace() {
                    // trim whitespace
                    self.scanner.advance_while(char::is_whitespace);
                    continue 'search;
                } else if x == '/' && peek == Some(&'/') {
                    // trim line comment
                    self.scanner.advance();
                    let documentation = Some(&'|') == self.scanner.chr();
                    self.scanner.advance_while(|x| x != '\n');
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
                } else if x.is_ascii_digit() {
                    // lex numbers
                    let mut is_real = false;
                    while let Some(x) = self.scanner.chr() {
                        if let '.' = x {
                            if is_real {
                                break;
                            } else {
                                is_real = true;
                            }
                        } else if !x.is_ascii_digit() {
                            break;
                        }
                        self.scanner.advance();
                    }
                    TokenKind::Literal(if is_real {
                        LiteralKind::Real
                    } else {
                        LiteralKind::Integer
                    })
                } else if x == '_' || x.is_alphanumeric() {
                    // lex keywords and identifiers
                    self.scanner.advance_while(|x| x == '_' || x.is_alphanumeric());
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
                } else {
                    // lex symbols
                    match x {
                        '(' => TokenKind::Symbol(SymbolKind::LeftParen),
                        ')' => TokenKind::Symbol(SymbolKind::RightParen),
                        '{' => TokenKind::Symbol(SymbolKind::LeftBrace),
                        '}' => TokenKind::Symbol(SymbolKind::RightBrace),
                        '[' => TokenKind::Symbol(SymbolKind::LeftBox),
                        ']' => TokenKind::Symbol(SymbolKind::RightBox),
                        '.' => TokenKind::Symbol(SymbolKind::Dot),
                        '=' => TokenKind::Symbol(SymbolKind::Assign),
                        '\\' => TokenKind::Symbol(SymbolKind::Backslash),
                        ',' => TokenKind::Symbol(SymbolKind::Comma),
                        ':' => TokenKind::Symbol(SymbolKind::Colon),
                        ';' => TokenKind::Symbol(SymbolKind::SemiColon),
                        '$' => TokenKind::Symbol(SymbolKind::Dollar),
                        '`' => TokenKind::Symbol(SymbolKind::Backtick),
                        '"' => {
                            // lex string literal
                            loop {
                                if let Some(x) = self.scanner.advance() {
                                    if x == '\\' {
                                        self.scanner.advance();
                                    } else if x == '"' {
                                        break TokenKind::Literal(LiteralKind::String);
                                    }
                                } else {
                                    self.error("unterminated string literal");
                                    continue 'search;
                                }
                            }
                        },
                        _ => TokenKind::Operator(OperatorKind::Custom)
                    }
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