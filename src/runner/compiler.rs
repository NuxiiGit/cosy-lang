#![allow(dead_code)]

use super::data::{
    Position,
    error::*,
    syntax_tree::*
};

use std::iter::Peekable;
use std::str::CharIndices;
use std::collections::hash_map::HashMap;

macro_rules! matches {
    ($value:expr, $($pattern:tt)*) => ({
        match $value {
            $($pattern)* => true,
            _ => false
        }
    });
}

pub struct Parser<'a, 'b> {
    scanner : Lexer<'a>,
    current : Option<Lex<'a>>,
    precedence : HashMap<&'b str, usize>
}
impl<'a, 'b> Parser<'a, 'b> {
    /// A constant which represents the maximum precedence levels.
    pub const PRECEDENCE_LEVELS : usize = 9;

    /// Creates a new parser from this string.
    pub fn from(scanner : Lexer<'a>) -> Self {
        let mut precedence = HashMap::new();
        precedence.insert("+", 1);
        precedence.insert("*", 2);
        Parser {
            scanner : scanner,
            current : None,
            precedence
        }
    }

    /// Consumes the parser and produces an abstract syntax tree.
    pub fn parse(mut self) -> Result<Expr<'a>, Error> {
        self.advance()?;
        self.parse_expr()
    }
    
    /// Parses an expression.
    fn parse_expr(&mut self) -> Result<Expr<'a>, Error> {
        self.parse_expr_binary(0)
    }

    /// Parses an expression.
    fn parse_expr_binary(&mut self, p : usize) -> Result<Expr<'a>, Error> {
        if p <= Self::PRECEDENCE_LEVELS {
            let mut top = self.parse_expr_binary(p + 1)?;
            while let Some(Lex { token : Token::Op(ident), position }) = self.current {
                if self.precedence.get(ident).unwrap_or(&Self::PRECEDENCE_LEVELS) != &p {
                    break;
                }
                self.advance()?;
                let bottom = self.parse_expr_binary(p + 1)?;
                top = Expr::Call {
                    ident : Ident { ident, position },
                    args : vec![top, bottom]
                }
            }
            Ok(top)
        } else {
            self.parse_expr_unary()
        }
    }

    /// Parses a stream of prefix unary operators.
    fn parse_expr_unary(&mut self) -> Result<Expr<'a>, Error> {
        if let Some(Lex { token : Token::Op(ident), position }) =
                self.consume(|x| matches!(x, Token::Op(..)))? {
            let right = self.parse_expr_unary()?;
            Ok(Expr::Call {
                ident : Ident { ident, position },
                args : vec![right]
            })
        } else {
            self.parse_expr_member()
        }
    }

    /// Parses a stream of member accesses.
    fn parse_expr_member(&mut self) -> Result<Expr<'a>, Error> {
        let mut expr = self.parse_expr_frontier()?;
        while let Some(Lex { token : Token::Ident(ident), position }) =
                self.consume(|x| matches!(x, Token::Ident(..)))? {
            expr = Expr::Member {
                ident : Ident { ident, position },
                expr : Box::new(expr)
            }
        }
        Ok(expr)
    }

    /// Parses expression literals and groupings.
    fn parse_expr_frontier(&mut self) -> Result<Expr<'a>, Error> {
        if let Some(Lex { token, position }) = 
                self.consume(|x| matches!(x,
                        Token::Str(..) |
                        Token::Int(..)))? {
            let result = match token {
                Token::Int(literal) => {
                    if let Ok(n) = literal.parse::<i64>() {
                        Ok(Value::Int(n))
                    } else {
                        Err("Unable to parse integer literal")
                    }
                },
                _ => Err("Unknown literal")
            };
            match result {
                Ok(value) => Ok(Expr::Literal { literal : Literal { value, position } }),
                Err(description) => Err(Error { description, position })
            }
        } else if let Some(Lex { token : Token::Ident(ident), position }) = 
                self.consume(|x| matches!(x, Token::Ident(..)))? {
            Ok(Expr::Variable { ident : Ident { ident, position } })
        } else {
            self.expects(|x| matches!(x, Token::LeftParen), "Malformed expression")?;
            let expr = self.parse_expr()?;
            self.expects(|x| matches!(x, Token::RightParen), "Expected closing ')' after expression")?;
            Ok(expr)
        }
    }

    /// Consumes the next token *only* if the predicate holds. Returns an error otherwise.
    fn expects(&mut self, pred : impl Fn(&Token<'a>) -> bool, on_error : &'static str) -> Result<Lex<'a>, Error> {
        let holds = self.holds(pred);
        let current = self.advance()?;
        if holds {
            Ok(current.unwrap())
        } else {
            // raise error
            let position = match current {
                Some(lex) => lex.position,
                _ => self.scanner.position()
            };
            Err(Error {
                description : on_error,
                position
            })
        }
    }

    /// Consumes the next token if the predicate holds.
    fn consume(&mut self, pred : impl Fn(&Token<'a>) -> bool) -> Result<Option<Lex<'a>>, Error> {
        if self.holds(pred) {
            self.advance()
        } else {
            Ok(None)
        }
    }

    /// Checks whether the current token has this property.
    fn holds(&self, pred : impl Fn(&Token<'a>) -> bool) -> bool {
        if let Some(Lex { token, .. }) = &self.current {
            pred(token)
        } else {
            false
        }
    }

    /// Advances the parser.
    fn advance(&mut self) -> Result<Option<Lex<'a>>, Error> {
        let previous = self.current.take();
        match self.scanner.next() {
            Some(Ok(lex)) => {
                self.current = Some(lex);
                Ok(previous)
            },
            Some(Err(e)) => {
                self.current = None;
                Err(e)
            },
            None => {
                self.current = None;
                Ok(previous)
            }
        }
    }
}

/// An iterator over a string slice, which produces `Token`s.
pub struct Lexer<'a> {
    context : &'a str,
    chars : Peekable<CharIndices<'a>>,
    position : Position
}
impl<'a> Lexer<'a> {
    /// Create a new scanner from this str slice.
    pub fn from(context : &'a str) -> Lexer<'a> {
        Lexer {
            context,
            chars : context
                    .char_indices()
                    .peekable(),
            position : (1, 1)
        }
    }

    /// Returns the position of the lexer.
    fn position(&self) -> Position {
        self.position
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
            self.position.0 += 1;
            self.position.1 = 1;
        } else {
            self.position.1 += 1;
        }
        Some(x)
    }
}
impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Lex<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut start = self.pos();
        let position = self.position;
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
        Some(match result {
            Ok(token) => Ok(Lex { token, position }),
            Err(description) => Err(Error { description, position })
        })
    }
}

/// A struct which stores a `Token` with the row and column it occured on.
pub struct Lex<'a> {
    pub token : Token<'a>,
    pub position : Position
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