use super::token::Token;

/// A type which represents the char iterator used by the lexer.
#[allow(dead_code)]
type Chars<'a> = std::iter::Peekable<std::str::Chars<'a>>;

/// Tokenises the input expression into a list of tokens `token::Token`.
/// # Errors
/// Returns `None` when the lexer was unable to tokenise this expression.
#[allow(dead_code)]
pub fn lex(expression : &str) -> Result<Vec<Token>, &'static str> {
    let mut tokens : Vec<Token> = Vec::new();
    let mut chars : Chars = expression
            .chars()
            .peekable();
    while let Some(ch) = &chars.next() {
        let next : Option<&char> = chars.peek();
        let token : Token = match ch {
            // match whitespace
            _ if ch.is_whitespace() => continue,
            // match comments
            '\'' if match next {
                Some('\'') => true,
                Some('{') => true,
                _ => false
            } => {
                if let Some('\'') = chars.next() {
                    while let Some(x) = chars.next() {
                        if x == '\n' {
                            break;
                        }
                    }
                } else {
                    while let Some(x) = chars.next() {
                        if x == '}' {
                            if let Some('\'') = chars.peek() {
                                chars.next();
                                break;
                            }
                        }
                    }
                }
                continue;
            }
            // match strings
            '"' => {
                let mut inner : String = String::new();
                let mut escape : bool = false;
                loop {
                    if let Some(x) = chars.next() {
                        if escape {
                            inner.push(x);
                            escape = false;
                        } else {
                            match x {
                                '"' => break,
                                '\\' => escape = true,
                                _ => inner.push(x)
                            }
                        }
                    } else {
                        return Err("Unclosed string!");
                    }
                }
                Token::Str(inner)
            }
            // match numbers
            _ if ch.is_numeric() => {
                let mut numb : String = ch.to_string();
                while let Some(&x) = chars.peek() {
                    if x.is_numeric() {
                        numb.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                Token::Int(numb)
            },
            // match keywords or identifiers
            _ if ch.is_alphabetic() || *ch == '_' => {
                let mut ident : String = ch.to_string();
                while let Some(x) = chars.peek() {
                    if x.is_alphanumeric() || *x == '_' {
                        ident.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                let phrase : &str = &ident;
                match phrase {
                    "var" => Token::Var,
                    "if" => Token::If,
                    "ifnot" => Token::IfNot,
                    "else" => Token::Else,
                    _ => Token::Identifier(ident)
                }
            }
            // match symbols
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            ';' => Token::SemiColon,
            // match operators
            '+' => Token::Plus,
            '-' => Token::Dash,
            '*' => Token::Star,
            '/' => Token::ForwardSlash,
            '\\' => Token::BackwardSlash,
            // match everything else
            _ => return Err("Unexpected symbol!")
        };
        tokens.push(token);
    }
    Ok(tokens)
}