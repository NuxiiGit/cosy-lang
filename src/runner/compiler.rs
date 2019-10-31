#![allow(dead_code)]

use std::fmt;
use std::error;
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
    scanner : Peekable<Lexer<'a>>
}
impl<'a> Parser<'a> {
    /// Create a new parser from this scanner.
    pub fn from(scanner : Lexer<'a>) -> Parser<'a> {
        Parser {
            scanner : scanner.peekable()
        }
    }

    /// Consumes the parser and produces an abstract syntax tree.
    pub fn parse(mut self) -> Result<Expr<'a>, Error> {
        self.parse_expr()
    }

    /// Parses an expression.
    fn parse_expr(&mut self) -> Result<Expr<'a>, Error> {
        let mut left = self.parse_expr_unary()?;
        while let Some(Lex { token : Token::Op(ident), position }) =
                self.consume(|x| matches!(x, Token::Op(..)))? {
            let right = self.parse_expr_unary()?;
            left = Expr::Call {
                ident,
                args : vec![left, right],
                position
            }
        }
        Ok(left)
    }

    /// Parses a stream of prefix unary operators.
    fn parse_expr_unary(&mut self) -> Result<Expr<'a>, Error> {
        if let Some(Lex { token : Token::Op(ident), position }) =
                self.consume(|x| matches!(x, Token::Op(..)))? {
            let right = self.parse_expr_unary()?;
            Ok(Expr::Call {
                ident,
                args : vec![right],
                position
            })
        } else {
            self.parse_expr_frontier()
        }
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
                Ok(value) => Ok(Expr::Literal { value, position }),
                Err(description) => Err(Error { description, position })
            }
        } else if let Some(Lex { token : Token::Ident(ident), position }) = 
                self.consume(|x| matches!(x, Token::Ident(..)))? {
            Ok(Expr::Variable { ident, position })
        } else {
            self.expects(|x| matches!(x, Token::LeftParen), "Malformed expression")?;
            let expr = self.parse_expr()?;
            self.expects(|x| matches!(x, Token::RightParen), "Expected closing ')' after expression")?;
            Ok(expr)
        }
    }

    /// Consumes the next token *only* if the predicate holds. Returns an error otherwise.
    fn expects(&mut self, pred : impl Fn(&Token<'a>) -> bool, on_error : &'static str) -> Result<Lex<'a>, Error> {
        let next = self.consume(pred)?;
        match next {
            Some(lex) => Ok(lex),
            None => {
                // raise error
                Err(match self.scanner.next() {
                    Some(Ok(lex)) => Error {
                        description : on_error,
                        position : lex.position
                    },
                    _ => Error {
                        description : "Unexpected error",
                        position : (0, 0)
                    }
                })
            }
        }
    }

    /// Consumes the next token if the predicate holds.
    fn consume(&mut self, pred : impl Fn(&Token<'a>) -> bool) -> Result<Option<Lex<'a>>, Error> {
        let consume = if let Some(x) = self.scanner.peek() {
            match &x {
                Ok(Lex { token, .. }) => pred(token),
                Err(..) => true
            }
        } else {
            false
        };
        if consume {
            let lex = self.scanner.next().unwrap()?;
            Ok(Some(lex))
        } else {
            Ok(None)
        }
    }
}

/// A recursive enum which stores expression information.
#[derive(Debug)]
pub enum Expr<'a> {
    Variable {
        ident : &'a str,
        position : Position
    },
    Literal {
        value : Value,
        position : Position
    },
    Member {
        ident : &'a str,
        expr : Box<Expr<'a>>,
        position : Position
    },
    Call {
        ident : &'a str,
        args : Vec<Expr<'a>>,
        position : Position
    }
}

/// An enum which stores data type values.
#[derive(Debug)]
pub enum Value {
    Empty,
    Bool(bool),
    Char(char),
    Int(i64),
    Float(f64)
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
    type Item = Result<Lex<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut start = self.pos();
        let position = (self.row, self.column);
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

/// A struct which stores a compiler error with the row and column it occured on.
#[derive(Debug)]
pub struct Error {
    pub description : &'static str,
    pub position : Position
}
impl fmt::Display for Error {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        let (row, column) = self.position;
        write!(f, "Compile error at (row. {}, col. {}): {}",
                row, column, self.description)
    }
}
impl error::Error for Error {}

/// A type alias for script locations. Read as `(row, column)`.
pub type Position = (usize, usize);

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