#![allow(dead_code)]

use super::Error;
use super::Result;
use super::super::collections::token::{
    Token,
    TokenType
};

use std::iter::Peekable;
use std::str::CharIndices;

/// An iterator over the `compiler::Result<Token>`s of a `str` slice.
pub struct Lexer<'a> {
    context : &'a str,
    char_indices : Peekable<CharIndices<'a>>,
    row : usize,
    column : usize
}
impl<'a> Lexer<'a> {
    /// Create a new scanner from this `str` slice.
    pub fn from(context : &'a str) -> Lexer<'a> {
        Lexer {
            context,
            char_indices : context
                    .char_indices()
                    .peekable(),
            row : 1,
            column : 1
        }
    }

    /// Peek at the next character.
    pub fn scanner_peek(&mut self) -> Option<&char> {
        let (_, x) = self.char_indices.peek()?;
        Some(x)
    }

    /// Peek at the next index. Returns `str.len()` if the end is reached.
    pub fn scanner_pos(&mut self) -> usize {
        if let Some((i, _)) = self.char_indices.peek() {
            *i
        } else {
            self.context.len()
        }
    }

    /// Move to the next character.
    pub fn scanner_next(&mut self) -> Option<char> {
        let (_, c) = self.char_indices.next()?;
        if c == '\n' {
            // move to new line
            self.row += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(c)
    }

    /// Returns `true` if this character is a valid whitespace symbol.
    pub fn check_whitespace(x : char) -> bool {
        x.is_control() || x.is_whitespace()
    }

    /// Returns `true` if this character is a valid number symbol.
    pub fn check_number(x : char) -> bool {
        x.is_ascii_digit()
    }

    /// Returns `true` if this character is a valid identifier symbol.
    pub fn check_character(x : char) -> bool {
        x == '_' || x.is_alphabetic() || Self::check_number(x)
    }

    /// Returns `true` if this character is a valid operator symbol.
    pub fn check_operator(x : char) -> bool {
        if let '`' | '"' | '{' | '}' | '[' | ']' = x {
            // reserved symbols
            false
        } else {
            !Self::check_character(x) && !Self::check_whitespace(x)
        }
    }
}
impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut start : usize = self.scanner_pos();
        let row : usize = self.row;
        let column : usize = self.column;
        Some(match match self.scanner_next()? {
            // ignore whitespace
            x if Self::check_whitespace(x) => {
                while let Some(x) = self.scanner_peek() {
                    if x.is_whitespace() {
                        self.scanner_next();
                    } else {
                        break;
                    }
                }
                return self.next();
            },
            // ignore comments
            '\'' => {
                if let Some('{') = self.scanner_peek() {
                    // block comment
                    loop {
                        if let Some(x) = self.scanner_next() {
                            if x == '}' {
                                if let Some('\'') = self.scanner_next() {
                                    return self.next();
                                }
                            }
                        } else {
                            break Err("Unclosed comment block");
                        }
                    }
                } else {
                    // line comment
                    while let Some(x) = self.scanner_next() {
                        if x == '\n' {
                            break;
                        }
                    }
                    return self.next();
                }
            },
            // match string literals
            '"' => {
                start = self.scanner_pos();
                loop {
                    let i = self.scanner_pos();
                    if let Some(x) = self.scanner_next() {
                        if x == '\\' {
                            self.scanner_next();
                        } else if x == '"' {
                            break Ok(TokenType::String(&self.context[start..i]));
                        }
                    } else {
                        break Err("Unclosed string");
                    }
                }
            },
            // match number literals
            x if Self::check_number(x) => {
                let end : usize = loop {
                    if let Some(&x) = self.scanner_peek() {
                        if Self::check_number(x) || x == '\'' {
                            self.scanner_next();
                            continue;
                        }
                    }
                    break self.scanner_pos();
                };
                Ok(TokenType::Integer(&self.context[start..end]))
            },
            // match keywords and identifiers
            x if Self::check_character(x) => {
                let end : usize = loop {
                    if let Some(&x) = self.scanner_peek() {
                        if Self::check_character(x) || x == '\'' {
                            self.scanner_next();
                            continue;
                        }
                    }
                    break self.scanner_pos();
                };
                Ok(match &self.context[start..end] {
                    "var" => TokenType::Var,
                    "if" => TokenType::If,
                    "ifnot" => TokenType::IfNot,
                    "else" => TokenType::Else,
                    x => TokenType::Identifier(x)
                })
            },
            // match symbols and operators
            x if Self::check_operator(x) => {
                let end : usize = loop {
                    if let Some(&x) = self.scanner_peek() {
                        if Self::check_operator(x) || x == '\'' {
                            self.scanner_next();
                            continue;
                        }
                    }
                    break self.scanner_pos();
                };
                Ok(match &self.context[start..end] {
                    ":" => TokenType::Colon,
                    ";" => TokenType::SemiColon,
                    x => TokenType::Operator(x)
                })
            }
            // match bracket types
            '(' => Ok(TokenType::LeftParen),
            ')' => Ok(TokenType::RightParen),
            '{' => Ok(TokenType::LeftBrace),
            '}' => Ok(TokenType::RightBrace),
            // error case
            _ => Err("Unknown symbol")
        } {
            Ok(flavour) => Ok(Token { flavour, row, column}),
            Err(description) => Err(vec![Error {description, row, column}])
        })
    }
}

/// A trait which can be implemented by structs to offer a
/// way of converting into a `Lexer` type.
pub trait Tokeniser<'a> {
    /// Constructs a new scanner.
    fn tokenise(&'a self) -> Lexer<'a>;
}
impl<'a> Tokeniser<'a> for str {
    fn tokenise(&'a self) -> Lexer<'a> {
        Lexer::from(self)
    }
}