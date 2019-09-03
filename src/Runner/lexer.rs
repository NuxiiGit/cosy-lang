    #![allow(dead_code)]

use super::token::*;
use super::scanner::*;

/// Tokenises the input expression into a list of tokens `token::Token<'a>`.
pub fn lex<'a>(expression : &'a str) -> Result<Vec<Token<'a>>, (&'static str, usize, usize)> {
    let mut tokens : Vec<Token<'a>> = Vec::new();
    let mut scanner : Scanner = Scanner::new(expression);
    macro_rules! push {
        ($flavour:expr) => ({
            let flavour : TokenType = $flavour;
            let (row, col) : (usize, usize) = scanner.position();
            push!(flavour, row, col);
        });
        ($flavour:expr, $row:expr, $col:expr) => ({
            let token : Token = Token::new($flavour, $row, $col);
            tokens.push(token);
        })
    }
    macro_rules! lexerror {
        ($msg:expr) => ({
            let (row, col) : (usize, usize) = scanner.position();
            return Err(($msg, row, col));
        })
    }
    while let Some(c) = scanner.next() {
        match c {
            // match whitespace
            x if x.is_whitespace() => {
                while let Some(x) = scanner.peek() {
                    if !x.is_whitespace() {
                        break;
                    } else {
                        scanner.next();
                    }
                }
            },
            // match comments
            '\'' if match scanner.peek() {
                Some('\'') | Some('{') => true,
                _ => false
            } => {
                if let Some('\'') = scanner.next() {
                    while let Some(x) = scanner.next() {
                        if x == '\n' {
                            break;
                        }
                    }
                } else {
                    while let Some(x) = scanner.next() {
                        if x == '}' {
                            if let Some('\'') = scanner.next() {
                                break;
                            }
                        }
                    }
                }
            }
            // match strings
            '"' => {
                scanner.drop(); // ignore first '"'
                loop {
                    match scanner.peek() {
                        Some('"') => {
                            break;
                        },
                        Some('\\') => {
                            scanner.next();
                        },
                        Some(_) => {},
                        None => {
                            lexerror!("Unclosed string!");
                        }
                    }
                    scanner.next();
                };
                push!(TokenType::String(scanner.slice()));
                scanner.next(); // ignore final '"'
            }
            // match numbers
            x if x.is_numeric() => {
                while let Some(x) = scanner.peek() {
                    if x.is_numeric() ||
                            *x == '_' {
                        scanner.next();
                    } else {
                        break;
                    }
                }
                push!(TokenType::Integer(scanner.slice()));
            },
            // match keywords and identifiers
            x if x.is_alphabetic() || x == '_' => {
                while let Some(x) = scanner.peek() {
                    if x.is_alphanumeric() ||
                            *x == '_' {
                        scanner.next();
                    } else {
                        break;
                    }
                }
                push!(match scanner.slice() {
                    "var" => TokenType::Var,
                    "if" => TokenType::If,
                    "ifnot" => TokenType::IfNot,
                    "else" => TokenType::Else,
                    x => TokenType::Identifier(x)
                });
            }
            // unknown symbol
            _ => {
                return lexerror!("Unknown symbol");
            }
        }
        scanner.drop();
    }
    Ok(tokens)
}