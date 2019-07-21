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
            &x if x.is_whitespace() => continue,
            &x if x.is_alphabetic() || x == '_' => {
                // keyword or identifier
                let mut s : String = x.to_string();
                while let Some(&x) = chars.peek() {
                    if x.is_alphanumeric() || x == '_' {
                        s.push(x);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if &s == "_" || 
                        &s == "var" ||
                        &s == "if" ||
                        &s == "ifnot" ||
                        &s == "then" ||
                        &s == "else" ||
                        &s == "repeat" ||
                        &s == "while" ||
                        &s == "until" ||
                        &s == "for" ||
                        &s == "step" ||
                        &s == "function" ||
                        &s == "return" ||
                        &s == "break" {
                    tokens.push(Token::new(&s, None))
                } else {
                    tokens.push(Token::new("identifier", Some(&s)))
                }
            },
            &x if x.is_numeric() => {
                // number
                let mut contains_point : bool = false;
                let mut s : String = x.to_string();
                while let Some(&x) = chars.peek() {
                    if x.is_numeric() ||
                            (!contains_point && x == '.') {
                        s.push(x);
                        chars.next();
                        if x == '.' {
                            contains_point = true;
                        }
                    } else {
                        break;
                    }
                }
                tokens.push(Token::new("number", Some(&s)));
            },
            &x if x == '(' || x == ')' ||
                    x == '{' || x == '}' ||
                    x == '[' || x == ']' ||
                    x == ';'=> tokens.push(Token::new(&x.to_string(), None)),
            '\'' => {
                match chars.peek() {
                    Some('\'') => {
                        // single line comment
                        chars.next();
                        while let Some(x) = chars.next() {
                            if x == '\n' {
                                break;
                            }
                        }
                        continue;
                    },
                    Some('{') => {
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
                    Some(&x) if x.is_alphabetic() || x == '_' => {
                        // label
                        let mut s : String = x.to_string();
                        chars.next();
                        while let Some(&x) = chars.peek() {
                            if x.is_alphanumeric() || x == '_' {
                                s.push(x);
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        tokens.push(Token::new("label", Some(&s)));
                    }
                    _ => return None
                }
            },
            '"' => {
                // string
                let mut escape : bool = false;
                let mut s : String = String::new();
                loop {
                    if let Some(x) = chars.next() {
                        match x {
                            '\\' => escape = true,
                            '"' if !escape => break,
                            x => s.push(x)
                        }
                    } else {
                        return None;
                    }
                }
                tokens.push(Token::new("string", Some(&s)));
            },
            _ => return None
        }
    }
    Some(tokens)
}