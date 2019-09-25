#![allow(dead_code)]

use super::essentials::{
    token::*,
    error::*
};

use std::iter::Peekable;
use std::str::CharIndices;

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
    fn next_char(&mut self) -> Option<char> {
        let (_, x) : (_, char) = self.next_charindex()?;
        Some(x)
    }

    /// Move to the next character-index pair.
    fn next_charindex(&mut self) -> Option<(usize, char)> {
        let next : (usize, char) = self.scanner.next()?;
        if let (_, '\n') = next {
            self.row += 1;
            self.column = 1;
        } else {
            self.column = 1;
        }
        Some(next)
    }

    /// Peek at the next character.
    fn peek_char(&mut self) -> Option<char> {
        let (_, x) = self.peek_charindex()?;
        Some(x)
    }

    /// Peek at the next index. Returns `context.len()` if the end is reached.
    fn peek_index(&mut self) -> usize {
        if let Some((i, _)) = self.peek_charindex() {
            i
        } else {
            self.context.len()
        }
    }

    /// Peek at the next character-index pair.
    fn peek_charindex(&mut self) -> Option<(usize, char)> {
        let peek : &(usize, char) = self.scanner.peek()?;
        Some(*peek)
    }

    /// Create a new token with the current row and column numbers.
    fn token(&self, flavour : TokenType<'a>) -> Option<Token<'a>> {
        Some(Token {
            flavour,
            row : self.row,
            column : self.column
        })
    }

    /// Push an error onto the error list.
    fn error(&mut self, message : &'static str) {
        Error::new(message, self.row, self.column).throw();
    }
}
impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let (mut start, c) = self.next_charindex()?;
        match c {
            // leading whitespace
            x if x.is_whitespace() => {
                while let Some(x) = self.peek_char() {
                    if x.is_whitespace() {
                        self.next_char();
                    } else {
                        break;
                    }
                }
                self.next()
            }
            // comments
            '\'' => {
                if let Some('{') = self.peek_char() {
                    // block comment
                    loop {
                        if let Some(x) = self.next_char() {
                            if x == '}' {
                                if let Some('\'') = self.next_char() {
                                    break;
                                }
                            }
                        } else {
                            self.error("Unclosed comment block");
                            break;
                        }
                    }
                } else {
                    // line comment
                    while let Some(x) = self.next_char() {
                        if x == '\n' {
                            break;
                        }
                    }
                }
                self.next()
            },
            // match keywords and identifiers
            x if x.is_alphabetic() || x == '_' => {
                while let Some(x) = self.peek_char() {
                    if x.is_alphanumeric() ||
                            x == '_' ||
                            x == '\'' {
                        self.next_char();
                    } else {
                        break;
                    }
                }
                let end : usize = self.peek_index();
                self.token(match &self.context[start..end] {
                    "var" => TokenType::Var,
                    "if" => TokenType::If,
                    "ifnot" => TokenType::IfNot,
                    "else" => TokenType::Else,
                    x => TokenType::Identifier(x)
                })
            },
            // match string types
            x if x.is_quote() => {
                match x {
                    '"' => {
                        start = self.peek_index();
                        loop {
                            if let Some((i, x)) = self.next_charindex() {
                                if x == '\\' {
                                    self.next_char();
                                } else if x == '"' {
                                    break self.token(TokenType::String(
                                            &self.context[start..i]));
                                }
                            } else {
                                self.error("Unclosed string");
                                break self.next();
                            }
                        }
                    },
                    _ => {
                        self.error("Unknown quote type");
                        self.next()
                    }
                }
            },
            // match number types
            x if x.is_numeric() => {
                while let Some(x) = self.peek_char() {
                    if x.is_numeric() ||
                            x == '\'' {
                        self.next_char();
                    } else {
                        break;
                    }
                }
                let end : usize = self.peek_index();
                self.token(TokenType::Integer(
                        &self.context[start..end]))
            },
            // match bracket types
            x if x.is_bracket() => {
                if let Some(flavour) = match x {
                    '(' => Some(TokenType::LeftParen),
                    ')' => Some(TokenType::RightParen),
                    '{' => Some(TokenType::LeftBrace),
                    '}' => Some(TokenType::RightBrace),
                    _ => None
                } {
                    self.token(flavour)
                } else {
                    self.error("Unknown bracket type");
                    self.next()
                }
            },
            // match symbols and operators
            x if x.is_symbol() => {
                while let Some(x) = self.peek_char() {
                    if x.is_symbol() &&
                            !x.is_bracket() &&
                            !x.is_quote() {
                        self.next_char();
                    } else {
                        break;
                    }
                }
                let end : usize = self.peek_index();
                self.token(match &self.context[start..end] {
                    ":" => TokenType::Colon,
                    ";" => TokenType::SemiColon,
                    x => TokenType::Operator(x)
                })
            }
            // match nothing
            _ => {
                self.error("Unknown symbol");
                self.next()
            }
        }
    }
}

/// Additional methods for `char`
trait CharExt {
    /// Returns `true` if this `char` is a bracket.
    /// These include: `( )`, `{ }`, and `[ ]`.
    fn is_bracket(&self) -> bool;

    /// Returns `true` if this `char` is a symbol.
    fn is_quote(&self) -> bool;

    /// Returns `true` if this `char` is a symbol.
    fn is_symbol(&self) -> bool;
}
impl CharExt for char {
    fn is_bracket(&self) -> bool {
        if let '(' | ')' |
                '{' | '}' |
                '[' | ']' = self {
            true
        } else {
            false
        }
    }

    fn is_quote(&self) -> bool {
        if let '\'' | '"' | '`' = self {
            true
        } else {
            false
        }
    }

    fn is_symbol(&self) -> bool {
        !(self.is_alphanumeric() || self.is_whitespace())
    }
}