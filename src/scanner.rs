use super::source_pos::Span;
use super::error::Error;
use super::token::{
    Token,
    TokenKind,
    IdentifierKind,
    LiteralKind
};

use std::str::CharIndices;

/// An iterator over a string slice, which produces `Token`s.
pub struct Lexer<'a> {
    scanner : StrScanner<'a>
}
impl<'a> Lexer<'a> {
    /// Create a new lexer.
    pub fn lex(scanner : StrScanner<'a>) -> Lexer<'a> {
        Lexer { scanner }
    }
}
impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, Error<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.scanner.ignore();
        let row = self.scanner.row();
        let column = self.scanner.column();
        let result = match self.scanner.advance()? {
            // ignore whitespace
            x if x.is_whitespace() => {
                while let Some(x) = self.scanner.chr() {
                    if !x.is_whitespace() {
                        break;
                    }
                    self.scanner.advance();
                }
                return self.next();
            },
            // ignore line comments
            '-' if Some('-') == self.scanner.chr() => {
                while let Some(x) = self.scanner.advance() {
                    if x == '\n' {
                        break;
                    }
                }
                return self.next();
            },
            // ignore block comments
            '{' if Some('-') == self.scanner.chr() => {
                let mut nests = 1;
                while let Some(x) = self.scanner.advance() {
                    match x {
                        '-' if Some('}') == self.scanner.chr() => {
                            if nests == 1 {
                                self.scanner.advance();
                                return self.next();
                            } else {
                                nests -= 1;
                            }
                        },
                        '{' if Some('-') == self.scanner.chr() => {
                            nests += 1;
                        },
                        _ => continue
                    }
                }
                Err("unterminated block comment")
            },
            // match keywords and identifiers
            x if x.is_alphabetic() => {
                while let Some(x) = self.scanner.chr() {
                    if !(x.is_alphanumeric() || x == '_' || x == '\'') {
                        break;
                    }
                    self.scanner.advance();
                }
                Ok(match self.scanner.substr() {
                    "var" => TokenKind::Var,
                    "if" => TokenKind::If,
                    "ifnot" => TokenKind::IfNot,
                    "else" => TokenKind::Else,
                    _ => TokenKind::Identifier(IdentifierKind::Alphanumeric)
                })
            },
            // match number types
            x if x.is_ascii_digit() => {
                while let Some(x) = self.scanner.chr() {
                    if !(x.is_ascii_digit() || x == '_') {
                        break;
                    }
                    self.scanner.advance();
                }
                Ok(TokenKind::Literal(LiteralKind::Integer))
            },
            // unknown lex
            _ => Err("unexpected symbol")
        };
        let span = Span { content : self.scanner.substr(), row, column };
        Some(match result {
            Ok(kind) => Ok(Token { kind, span }),
            Err(reason) => Err(Error { reason, span })
        })
    }
}

/// A structure over a string slice which produces individual `Span`s of tokens.
pub struct StrScanner<'a> {
    context : &'a str,
    chars : CharIndices<'a>,
    peeked : Option<char>,
    row : usize,
    column : usize,
    span_begin : usize,
    span_end : usize
}
impl<'a> StrScanner<'a> {
    /// Create a new scanner from this string slice.
    pub fn from(context : &'a str) -> StrScanner<'a> {
        let mut chars = context.char_indices();
        let peeked = if let Some((_, x)) = chars.next() {
            // get the first character
            // this allows for the string scanner to have an immutable `chr` method
            Some(x)
        } else {
            None
        };
        StrScanner {
            context,
            chars,
            peeked,
            row : 1,
            column : 1,
            span_begin : 0,
            span_end : 0,
        }
    }

    /// Returns the current column of the scanner.
    pub fn column(&self) -> usize {
        self.column
    }

    /// Returns the current row of the scanner.
    pub fn row(&self) -> usize {
        self.row
    }

    /// Peeks at the current substring.
    pub fn substr(&mut self) -> &'a str {
        &self.context[self.span_begin..self.span_end]
    }

    /// Erases the current substring.
    pub fn ignore(&mut self) {
        self.span_begin = self.span_end;
    }

    /// Peek at the next character.
    pub fn chr(&self) -> Option<char> {
        self.peeked
    }

    /// Move to the next character.
    pub fn advance(&mut self) -> Option<char> {
        let previous = self.chr();
        self.peeked = if let Some((i, x)) = self.chars.next() {
            // update span
            self.span_end = i;
            // move to new line
            if x == '\n' {
                self.row += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            Some(x)
        } else {
            // end of file
            self.span_end = self.context.len();
            None
        };
        previous
    }
}