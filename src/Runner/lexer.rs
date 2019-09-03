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
            push!(flavour, scanner.row(), scanner.column());
        });
        ($flavour:expr, $row:expr, $col:expr) => ({
            let token : Token = Token::new($flavour, $row, $col);
            tokens.push(token);
        })
    }
    macro_rules! lexerror {
        ($msg:expr) => ({
            return Err(($msg, scanner.row(), scanner.column()));
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
                let start : usize = scanner.index_right(); // ignore first '"'
                loop {
                    if let Some(x) = scanner.next() {
                        if x == '\\' {
                            scanner.next();
                        } else if x == '"' {
                            break;
                        }
                    } else {
                        lexerror!("Unclosed string!");
                    }
                };
                let end : usize = scanner.index_left(); // ignore final '"'
                push!(TokenType::String(scanner.slice(start, end)));
            }
            // match numbers
            x if x.is_numeric() => {
                let start : usize = scanner.index_left();
                while let Some(x) = scanner.peek() {
                    if x.is_numeric() ||
                            *x == '_' {
                        scanner.next();
                    } else {
                        break;
                    }
                }
                let end : usize = scanner.index_right();
                push!(TokenType::Integer(scanner.slice(start, end)));
            },
            // match keywords and identifiers
            x if x.is_alphabetic() || x == '_' => {
                let start : usize = scanner.index_left();
                while let Some(x) = scanner.peek() {
                    if x.is_alphanumeric() ||
                            *x == '_' {
                        scanner.next();
                    } else {
                        break;
                    }
                }
                let end : usize = scanner.index_right();
                push!(match scanner.slice(start, end) {
                    "var" => TokenType::Var,
                    "if" => TokenType::If,
                    "ifnot" => TokenType::IfNot,
                    "else" => TokenType::Else,
                    x => TokenType::Identifier(x)
                });
            },
            // unknown symbol
            _ => {
                return lexerror!("Unknown symbol");
            }
        }
    }
    Ok(tokens)
}