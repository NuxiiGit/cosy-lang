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
        // trim whitespace
        while valid_whitespace(self.scanner.chr()?) {
            self.scanner.advance();
        }
        self.scanner.drop_substr();
        // begin
        let row = self.scanner.row();
        let column = self.scanner.column();
        let result = match self.scanner.advance()? {
            // match number types
            x if valid_number(x) => {
                while let Some(x) = self.scanner.chr() {
                    if valid_number(x) {
                        self.scanner.advance();
                    } else {
                        break;
                    }
                }
                Ok(TokenKind::Literal(LiteralKind::Integer))
            },
            // match keywords and identifiers
            x if valid_character(x) => {
                while let Some(x) = self.scanner.chr() {
                    if valid_character(x) {
                        self.scanner.advance();
                    } else {
                        continue;
                    }
                }
                Ok(match self.scanner.peek_substr() {
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
        let span = Span { content : self.scanner.take_substr(), row, column };
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
    seeker : usize
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
            seeker : 0
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

    /// Returns the current substring.
    pub fn take_substr(&mut self) -> &'a str {
        let s = self.peek_substr();
        self.drop_substr();
        s
    }

    /// Peeks at the current substring.
    pub fn peek_substr(&mut self) -> &'a str {
        &self.context[self.seeker..self.pos()]
    }

    /// Ignores the current substring.
    pub fn drop_substr(&mut self) {
        self.seeker = self.pos();
    }

    /// Peek at the next character.
    pub fn chr(&mut self) -> Option<char> {
        let (_, x) = self.chars.peek()?;
        Some(*x)
    }

    /// Returns the position of the current character.
    pub fn pos(&mut self) -> usize {
        if let Some((i, _)) = self.chars.peek() {
            *i
        } else {
            self.context.len()
        }
    }

    /// Move to the next character.
    pub fn advance(&mut self) -> Option<char> {
        let (_, x) = self.chars.next()?;
        if x == '\n' {
            // move to new line
            self.row += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(x)
    }
}