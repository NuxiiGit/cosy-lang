use super::source_pos::Span;
use super::error::Error;
use super::token::{
    Token,
    TokenKind,
    IdentifierKind,
    LiteralKind
};

use std::iter::Peekable;
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
        self.scanner.erase();
        let row = self.scanner.row();
        let column = self.scanner.column();
        let result = match self.scanner.advance()? {
            // ignore whitespace
            x if valid_whitespace(x) => {
                while let Some(x) = self.scanner.chr() {
                    if valid_whitespace(x) {
                        self.scanner.advance();
                    } else {
                        break;
                    }
                }
                return self.next();
            },
            // match quote types
            x if valid_quote(x) => {
                match x {
                    _ => Err("unexpected quote type")
                }
            },
            // match number types
            x if valid_number(x) => {
                while let Some(x) = self.scanner.chr() {
                    if !valid_number(x) {
                        break;
                    }
                    self.scanner.advance();
                }
                Ok(TokenKind::Literal(LiteralKind::Integer))
            },
            // match keywords and identifiers
            x if valid_character(x) => {
                while let Some(x) = self.scanner.chr() {
                    if !valid_character(x) {
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
            }
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

/// Returns `true` if this character is a valid whitespace symbol.
fn valid_whitespace(x : char) -> bool {
    x.is_control() || x.is_whitespace()
}

/// Returns `true` if this character is a valid number symbol.
fn valid_number(x : char) -> bool {
    x == '\'' || x.is_ascii_digit()
}

/// Returns `true` if this character is a valid identifier symbol.
fn valid_character(x : char) -> bool {
    x == '_' || x.is_alphabetic() || valid_number(x)
}

/// Returns `true` if this character is a valid bracket symbol.
fn valid_bracket(x : char) -> bool {
    if let '{' | '}' | '[' | ']' | '(' | ')' = x {
        true
    } else {
        false
    }
}

/// Returns `true` if this character is a valid quote symbol.
fn valid_quote(x : char) -> bool {
    if let '"' | '\'' | '`' = x {
        true
    } else {
        false
    }
}

/// Returns `true` if this character is a valid operator symbol.
fn valid_operator(x : char) -> bool {
    !(valid_character(x) || valid_whitespace(x) ||
            valid_bracket(x) || valid_quote(x))
}

/// A structure over a string slice which produces individual `Span`s of tokens.
pub struct StrScanner<'a> {
    context : &'a str,
    chars : Peekable<CharIndices<'a>>,
    row : usize,
    column : usize,
    span_begin : usize,
    span_end : usize
}
impl<'a> StrScanner<'a> {
    /// Create a new scanner from this string slice.
    pub fn from(context : &'a str) -> StrScanner<'a> {
        StrScanner {
            context,
            chars : context
                    .char_indices()
                    .peekable(),
            row : 1,
            column : 0,
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
    pub fn erase(&mut self) {
        self.span_begin = self.span_end;
    }

    /// Peek at the next character.
    pub fn chr(&mut self) -> Option<char> {
        let (_, x) = self.chars.peek()?;
        Some(*x)
    }

    /// Move to the next character.
    pub fn advance(&mut self) -> Option<char> {
        let (_, x) = self.chars.next()?;
        // move to new line
        if x == '\n' {
            self.row += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        // update span
        self.span_end = if let Some((i, _)) = self.chars.peek() {
            *i
        } else {
            self.context.len()
        };
        Some(x)
    }
}