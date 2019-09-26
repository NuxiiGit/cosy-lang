#![allow(dead_code)]

use super::Result;
use super::collections::token::*;

use std::iter::Peekable;
use std::str::CharIndices;
use std::error;
use std::fmt;

/// A struct which encapsulates the state of the scanner.
pub struct Lexer<'a> {
    context : &'a str,
    scanner : Peekable<CharIndices<'a>>,
    row : usize,
    column : usize
}
impl<'a> Lexer<'a> {
    /// Constructs a new scanner.
    pub fn new(context : &'a str) -> Lexer<'a> {
        Lexer {
            context,
            scanner : context
                    .char_indices()
                    .peekable(),
            row : 1,
            column : 1
        }
    }

    /// Move to the next character.
    fn char_next(&mut self) -> Option<char> {
        let (_, c) = self.scanner.next()?;
        if c == '\n' {
            // move to new line
            self.row += 1;
            self.column = 1;
        } else {
            self.column = 1;
        }
        Some(c)
    }

    /// Peek at the next character.
    fn char_peek(&mut self) -> Option<char> {
        let (_, x) = self.scanner.peek()?;
        Some(*x)
    }

    /// Peek at the next index. Returns `context.len()` if the end is reached.
    fn char_index(&mut self) -> usize {
        if let Some((i, _)) = self.scanner.peek() {
            *i
        } else {
            self.context.len()
        }
    }

    fn test() -> bool {
        true
    }
}
impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        macro_rules! error {
            ($description:expr) => ({
                Some(Err(Box::new(LexerError {
                    description : $description,
                    row : self.row,
                    column : self.column
                })))
            });
        }
        macro_rules! token {
            ($flavour:expr) => ({
                Some(Ok(Token {
                    flavour : $flavour,
                    row : self.row,
                    column : self.column
                }))
            });
        }
        macro_rules! valid_operator {
            ($c:expr) => ({
                if let '!' | '%' | '&' |
                        '*' | '+' | '-' |
                        '.' | '/' | ':' |
                        ';' | '<' | '=' |
                        '>' | '?' | '\\' |
                        '^' | '|' | '~' = $c {
                    true
                } else {
                    false
                }
            });
        }
        let mut start = self.char_index();
        match self.char_next()? {
            // remove whitespace
            x if x.is_whitespace() => {
                while let Some(x) = self.char_peek() {
                    if x.is_whitespace() {
                        self.char_next();
                    } else {
                        break;
                    }
                }
                self.next()
            },
            // comments
            '\'' => {
                if let Some('{') = self.char_peek() {
                    // block comment
                    loop {
                        if let Some(x) = self.char_next() {
                            if x == '}' {
                                if let Some('\'') = self.char_next() {
                                    break;
                                }
                            }
                        } else {
                            return error!("Unclosed comment block");
                        }
                    }
                } else {
                    // line comment
                    while let Some(x) = self.char_next() {
                        if x == '\n' {
                            break;
                        }
                    }
                }
                self.next()
            },
            // match string types
            '"' => {
                start = self.char_index();
                loop {
                    let i = self.char_index();
                    if let Some(x) = self.char_next() {
                        if x == '\\' {
                            self.char_next();
                        } else if x == '"' {
                            break token!(TokenType::String(
                                    &self.context[start..i]));
                        }
                    } else {
                        break error!("Unclosed string");
                    }
                }
            },
            // match keywords and identifiers
            'A'..='Z' | '_' | 'a'..='z' => {
                while let Some(x) = self.char_peek() {
                    if x.is_alphanumeric() ||
                            x == '_' ||
                            x == '\'' {
                        self.char_next();
                    } else {
                        break;
                    }
                }
                let end : usize = self.char_index();
                token!(match &self.context[start..end] {
                    "var" => TokenType::Var,
                    "if" => TokenType::If,
                    "ifnot" => TokenType::IfNot,
                    "else" => TokenType::Else,
                    x => TokenType::Identifier(x)
                })
            },
            // match number types
            '0'..='9' => {
                while let Some(x) = self.char_peek() {
                    if x.is_numeric() ||
                            x == '\'' {
                        self.char_next();
                    } else {
                        break;
                    }
                }
                let end : usize = self.char_index();
                token!(TokenType::Integer(
                        &self.context[start..end]))
            },
            // match bracket types
            '(' => token!(TokenType::LeftParen),
            ')' => token!(TokenType::RightParen),
            '{' => token!(TokenType::LeftBrace),
            '}' => token!(TokenType::RightBrace),
            // match symbols and operators
            x if valid_operator!(x) => {
                while let Some(x) = self.char_peek() {
                    if valid_operator!(x) ||
                            x == '\'' ||
                            x == '_' {
                        self.char_next();
                    } else {
                        break;
                    }
                }
                let end : usize = self.char_index();
                token!(match &self.context[start..end] {
                    ":" => TokenType::Colon,
                    ";" => TokenType::SemiColon,
                    x => TokenType::Operator(x)
                })
            },
            // match error
            _ => error!("Unknown symbol")
        }
    }
}

/// An error type which represents a lexer error.
#[derive(Debug)]
pub struct LexerError {
    pub description : &'static str,
    pub row : usize,
    pub column : usize
}
impl fmt::Display for LexerError {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Scanner error at (row. {}, col. {}): {}",
                self.row, self.column, self.description)
    }
}
impl error::Error for LexerError {}