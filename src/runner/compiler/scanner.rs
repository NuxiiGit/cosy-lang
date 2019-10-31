#![allow(dead_code)]

use super::super::collections::Token;

use std::iter::Peekable;
use std::str::CharIndices;

/// An iterator over a string slice, which produces `Token`s.
pub struct Lexer<'a> {
    context : &'a str,
    chars : Peekable<CharIndices<'a>>,
    row : usize,
    column : usize
}
impl<'a> Lexer<'a> {
    /// Create a new scanner from this str slice.
    pub fn from(context : &'a str) -> Lexer<'a> {
        Lexer {
            context,
            chars : context
                    .char_indices()
                    .peekable(),
            row : 1,
            column : 1
        }
    }

    /// Peek at the next character.
    fn chr(&mut self) -> Option<char> {
        let (.., x) = self.chars.peek()?;
        Some(*x)
    }

    /// Peek at the next index. Returns `str.len()` if the end is reached.
    fn pos(&mut self) -> usize {
        if let Some((i, ..)) = self.chars.peek() {
            *i
        } else {
            self.context.len()
        }
    }

    /// Move to the next character.
    fn advance(&mut self) -> Option<char> {
        let (.., x) = self.chars.next()?;
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
impl<'a> Iterator for Lexer<'a> {
    type Item = ((usize, usize), Result<Token<'a>, &'static str>);

    fn next(&mut self) -> Option<Self::Item> {
        let mut start = self.pos();
        let position = (self.row, self.column);
        let result = match self.advance()? {
            // ignore whitespace
            x if check_whitespace(x) => {
                while let Some(x) = self.chr() {
                    if x.is_whitespace() {
                        self.advance();
                    } else {
                        break;
                    }
                }
                return self.next();
            },
            // match quote types
            x if check_quote(x) => {
                match x {
                    // ignore comments
                    '\'' => {
                        if let Some('{') = self.chr() {
                            // block comment
                            self.advance();
                            while let Some(x) = self.advance() {
                                if x == '}' {
                                    if let Some('\'') = self.advance() {
                                        return self.next();
                                    }
                                }
                            }
                            Err("Unclosed comment block")
                        } else {
                            // line comment
                            while let Some(x) = self.advance() {
                                if x == '\n' {
                                    break;
                                }
                            }
                            return self.next();
                        }
                    },
                    // string literal
                    '"' => {
                        start = self.pos();
                        loop {
                            let end = self.pos();
                            if let Some(x) = self.advance() {
                                if x == '\\' {
                                    self.advance();
                                } else if x == '"' {
                                    break Ok(Token::Str(&self.context[start..end]));
                                }
                            } else {
                                break Err("Unclosed string");
                            }
                        }
                    },
                    // identifier literal
                    '`' => {
                        start = self.pos();
                        loop {
                            let end = self.pos();
                            if let Some(x) = self.advance() {
                                if x == '`' {
                                    break Ok(Token::Ident(&self.context[start..end]));
                                }
                            } else {
                                break Err("Unclosed identifier literal");
                            }
                        }
                    },
                    _ => Err("Unexpected quote symbol")
                }
            },
            // match bracket types
            x if check_bracket(x) => {
                match x {
                    '(' => Ok(Token::LeftParen),
                    ')' => Ok(Token::RightParen),
                    '{' => Ok(Token::LeftBrace),
                    '}' => Ok(Token::RightBrace),
                    _ => Err("Unexpected bracket symbol")
                }
            }
            // match number types
            x if check_number(x) => {
                let end = loop {
                    if let Some(x) = self.chr() {
                        if check_number(x) {
                            self.advance();
                            continue;
                        }
                    }
                    break self.pos();
                };
                Ok(Token::Int(&self.context[start..end]))
            },
            // match keywords and identifiers
            x if check_character(x) => {
                let end = loop {
                    if let Some(x) = self.chr() {
                        if check_character(x) {
                            self.advance();
                            continue;
                        }
                    }
                    break self.pos();
                };
                Ok(match &self.context[start..end] {
                    "var" => Token::Var,
                    "if" => Token::If,
                    "ifnot" => Token::IfNot,
                    "else" => Token::Else,
                    x => Token::Ident(x)
                })
            },
            // match symbols and operators
            x if check_operator(x) => {
                let end = loop {
                    if let Some(x) = self.chr() {
                        if check_operator(x) {
                            self.advance();
                            continue;
                        }
                    }
                    break self.pos();
                };
                Ok(match &self.context[start..end] {
                    ":" => Token::Colon,
                    ";" => Token::SemiColon,
                    x => Token::Op(x)
                })
            }
            // what in the god damn
            _ => Err("Unexpected character")
        };
        Some((position, result))
    }
}

/// Returns `true` if this character is a valid whitespace symbol.
fn check_whitespace(x : char) -> bool {
    x.is_control() || x.is_whitespace()
}

/// Returns `true` if this character is a valid number symbol.
fn check_number(x : char) -> bool {
    x == '\'' || x.is_ascii_digit()
}

/// Returns `true` if this character is a valid identifier symbol.
fn check_character(x : char) -> bool {
    x == '_' || x.is_alphabetic() || check_number(x)
}

/// Returns `true` if this character is a valid bracket symbol.
fn check_bracket(x : char) -> bool {
    if let '{' | '}' | '[' | ']' | '(' | ')' = x {
        true
    } else {
        false
    }
}

/// Returns `true` if this character is a valid bracket symbol.
fn check_quote(x : char) -> bool {
    if let '"' | '\'' | '`' = x {
        true
    } else {
        false
    }
}

/// Returns `true` if this character is a valid operator symbol.
fn check_operator(x : char) -> bool {
    !(check_character(x) || check_whitespace(x) ||
            check_bracket(x) || check_quote(x))
}