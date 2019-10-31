#![allow(dead_code)]

use std::iter::Peekable;
use std::str::CharIndices;

macro_rules! matches {
    ($value:expr, $($pattern:tt)*) => ({
        match $value {
            $($pattern)* => true,
            _ => false
        }
    });
}

pub struct Parser<'a> {
    scanner : Lexer<'a>,
    
}

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
    type Item = Lex<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut start = self.pos();
        let row = self.row;
        let column = self.column;
        let result = match self.advance()? {
            // ignore whitespace
            x if x.valid_whitespace() => {
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
            x if x.valid_quote() => {
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
            x if x.valid_bracket() => {
                match x {
                    '(' => Ok(Token::LeftParen),
                    ')' => Ok(Token::RightParen),
                    '{' => Ok(Token::LeftBrace),
                    '}' => Ok(Token::RightBrace),
                    _ => Err("Unexpected bracket symbol")
                }
            }
            // match number types
            x if x.valid_number() => {
                let end = loop {
                    if let Some(x) = self.chr() {
                        if x.valid_number() {
                            self.advance();
                            continue;
                        }
                    }
                    break self.pos();
                };
                Ok(Token::Int(&self.context[start..end]))
            },
            // match keywords and identifiers
            x if x.valid_character() => {
                let end = loop {
                    if let Some(x) = self.chr() {
                        if x.valid_character() {
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
            x if x.valid_operator() => {
                let end = loop {
                    if let Some(x) = self.chr() {
                        if x.valid_operator() {
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
        Some(Lex { result, row, column })
    }
}

/// A struct which stores either a `Token` or a message `&str` depending on whether the lex failed or not.
/// Additionally, stores the row and column of the token/error.
pub struct Lex<'a> {
    pub result : Result<Token<'a>, &'static str>,
    pub row : usize,
    pub column : usize
}

/// An enum which describes available token types.
#[derive(Debug)]
pub enum Token<'a> {
    Var,
    If,
    IfNot,
    Else,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Colon,
    SemiColon,
    Str(&'a str),
    Int(&'a str),
    Ident(&'a str),
    Op(&'a str)
}

/// Extension functions for `char`.
trait CharExt {
    /// Returns `true` if this character is a valid whitespace symbol.
    fn valid_whitespace(&self) -> bool;

    /// Returns `true` if this character is a valid number symbol.
    fn valid_number(&self) -> bool;

    /// Returns `true` if this character is a valid identifier symbol.
    fn valid_character(&self) -> bool;

    /// Returns `true` if this character is a valid bracket symbol.
    fn valid_bracket(&self) -> bool;

    /// Returns `true` if this character is a valid quote symbol.
    fn valid_quote(&self) -> bool;

    /// Returns `true` if this character is a valid operator symbol.
    fn valid_operator(&self) -> bool;
}
impl CharExt for char {
    fn valid_whitespace(&self) -> bool {
        self.is_control() || self.is_whitespace()
    }

    fn valid_number(&self) -> bool {
        *self == '\'' || self.is_ascii_digit()
    }

    fn valid_character(&self) -> bool {
        *self == '_' || self.is_alphabetic() || self.valid_number()
    }

    fn valid_bracket(&self) -> bool {
        if let '{' | '}' | '[' | ']' | '(' | ')' = self {
            true
        } else {
            false
        }
    }

    fn valid_quote(&self) -> bool {
        if let '"' | '\'' | '`' = self {
            true
        } else {
            false
        }
    }

    fn valid_operator(&self) -> bool {
        !(self.valid_character() || self.valid_whitespace() ||
                self.valid_bracket() || self.valid_quote())
    }
}