use super::token::Token;

/// A type which represents the char iterator used by the lexer.
#[allow(dead_code)]
type Chars<'a> = std::iter::Peekable<std::str::Chars<'a>>;

/// Tokenises the input expression into a list of tokens `token::Token`.
/// # Errors
/// Returns `None` when the lexer was unable to tokenise this expression.
#[allow(dead_code)]
pub fn lex(expression : &str) -> Option<Vec<Token>> {
    let mut tokens : Vec<Token> = Vec::new();
    let mut chars : Chars = expression
            .chars()
            .peekable();
    while let Some(ch) = &chars.next() {
        match ch {
            '(' => tokens.push(Token::new("(", None)),
            ')' => tokens.push(Token::new(")", None)),
            '{' => tokens.push(Token::new("{", None)),
            '}' => tokens.push(Token::new("}", None)),
            '[' => tokens.push(Token::new("[", None)),
            ']' => tokens.push(Token::new("]", None)),
            ';' => tokens.push(Token::new(";", None)),
            ':' => tokens.push(Token::new(":", None)),
            ',' => tokens.push(Token::new(".", None)),
            '.' => tokens.push(Token::new(".", None)),
            '?' => tokens.push(Token::new("?", None)),
            '!' => if let Some('=') = chars.peek() {
                tokens.push(Token::new("!=", None));
                chars.next();
            } else {
                tokens.push(Token::new("!", None));
            },
            '>' => if let Some('=') = chars.peek() {
                tokens.push(Token::new(">=", None));
                chars.next();
            } else {
                tokens.push(Token::new(">", None));
            },
            '<' => if let Some('=') = chars.peek() {
                tokens.push(Token::new("<=", None));
                chars.next();
            } else {
                tokens.push(Token::new("<", None));
            },
            '=' => match chars.peek() {
                Some('=') => {
                    tokens.push(Token::new("==", None));
                    chars.next();
                },
                Some('>') => {
                    tokens.push(Token::new("=>", None));
                    chars.next();
                },
                _ => tokens.push(Token::new("=", None))
            },
            '+' => if let Some('=') = chars.peek() {
                tokens.push(Token::new("+=", None));
                chars.next();
            } else {
                tokens.push(Token::new("+", None));
            },
            '-' => match chars.peek() {
                Some('=') => {
                    tokens.push(Token::new("-=", None));
                    chars.next();
                },
                Some('>') => {
                    tokens.push(Token::new("->", None));
                    chars.next();
                },
                _ => tokens.push(Token::new("-", None))
            },
            '*' => if let Some('=') = chars.peek() {
                tokens.push(Token::new("*=", None));
                chars.next();
            } else {
                tokens.push(Token::new("*", None));
            },
            '/' => if let Some('=') = chars.peek() {
                tokens.push(Token::new("/=", None));
                chars.next();
            } else {
                tokens.push(Token::new("/", None));
            },
            '%' => if let Some('=') = chars.peek() {
                tokens.push(Token::new("%=", None));
                chars.next();
            } else {
                tokens.push(Token::new("%", None));
            },
            '"' => {
                // string
                let mut value : String = String::new();
                loop {
                    if let Some(x) = chars.next() {
                        match x {
                            '\\' => {
                                if let Some('"') = chars.peek() {
                                    value.push('"');
                                    chars.next();
                                } else {
                                    value.push('\\');
                                }
                            },
                            '"' => break,
                            x => value.push(x)
                        }
                    } else {
                        return None;
                    }
                }
                tokens.push(Token::new("string", Some(&value)));
            },
            '\'' => if let Some(x) = chars.peek() {
                match x {
                    '\'' => {
                        // comment
                        chars.next();
                        while let Some(x) = chars.next() {
                            if x == '\n' {
                                break;
                            }
                        }
                        continue;
                    },
                    '{' => {
                        // multiline comment
                        chars.next();
                        while let Some(x) = chars.next() {
                            if x == '}' {
                                if let Some('\'') = chars.peek() {
                                    chars.next();
                                    break;
                                }
                            }
                        }
                        continue;
                    },
                    &x if x.is_alphabetic() || x == '_' => {
                        // label
                        let mut value : String = String::new();
                        while let Some(&x) = chars.peek() {
                            if x.is_alphanumeric() || x == '_' {
                                value.push(x);
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        tokens.push(Token::new("label", Some(&value)));
                    },
                    _ => return None
                }
            } else {
                return None;
            },
            &x if x.is_whitespace() => continue,
            &x if x.is_alphabetic() || x == '_' => {
                // keyword or identifier
                let mut value : String = x.to_string();
                while let Some(&x) = chars.peek() {
                    if x.is_alphanumeric() || x == '_' {
                        value.push(x);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if &value == "_"
                        || &value == "var"
                        || &value == "if"
                        || &value == "ifnot"
                        || &value == "then"
                        || &value == "else"
                        || &value == "repeat"
                        || &value == "while"
                        || &value == "until"
                        || &value == "for"
                        || &value == "next"
                        || &value == "break"
                        || &value == "function"
                        || &value == "return" {
                    tokens.push(Token::new(&value, None));
                } else {
                    tokens.push(Token::new("identifier", Some(&value)));
                }
            },
            &x if x.is_numeric() => {
                // int or float
                let mut float : bool = false;
                let mut value : String = x.to_string();
                while let Some(&x) = chars.peek() {
                    match x {
                        '.' => {
                            if float {
                                break;
                            }
                            float = true;
                            value.push('.');
                            chars.next();
                        },
                        x if x.is_numeric() => {
                            value.push(x);
                            chars.next();
                        }
                        _ => break
                    }
                }
                tokens.push(Token::new(if float {"float"} else {"integer"}, Some(&value)));
            },
            _ => return None
        }
    }
    Some(tokens)
}