use super::interface::lexer::Chars;
use super::interface::lexer::Lexer;

/// Generates the language specific lexer.
pub fn generate_lexer() -> Lexer {
    Lexer::new()
            .ignore(lex_whitespace!())
            // comments
            .ignore(lex_line!("''"))
            .ignore(lex_region!("'{", "}'"))
            // blocks
            .add("(", lex_keyword!("("))
            .add(")", lex_keyword!(")"))
            .add("{", lex_keyword!("{"))
            .add("}", lex_keyword!("}"))
            // keywords
            .add("var", lex_keyword!("var"))
            .add("if", lex_keyword!("if"))
            .add("ifnot", lex_keyword!("ifnot"))
            .add("then", lex_keyword!("then"))
            .add("else", lex_keyword!("else"))
            .add("repeat", lex_keyword!("repeat"))
            .add("while", lex_keyword!("while"))
            .add("until", lex_keyword!("until"))
            .add("for", lex_keyword!("for"))
            .add("step", lex_keyword!("step"))
            .add("function", lex_keyword!("function"))
            .add("return", lex_keyword!("return"))
            // operators
            .add(":", lex_keyword!(":"))
            .add("=", lex_keyword!("="))
            .add("+", lex_keyword!("+"))
            .add("-", lex_keyword!("-"))
            .add("*", lex_keyword!("*"))
            .add("/", lex_keyword!("/"))
            // identifiers
            .add("IDENTIFIER", |chars| find_identifier(chars))
            .add("LABEL", |chars| {
                if let Some('\'') = chars.next() {
                    find_identifier(chars)
                } else {
                    None
                }
            })
            .add("STRING", lex_region!("\""))
            .add("NUMBER", |chars| {
                let mut contains_point : bool = false;
                let mut numb : String = String::new();
                while match chars.peek() {
                    Some(&ch) => {
                        if ch == '.' {
                            if contains_point {
                                // cannot have multiple points
                                return None;
                            }
                            contains_point = true;
                            true
                        } else {
                            ch.is_numeric()
                        }
                    },
                    None => false
                } {
                    numb.push(chars.next().unwrap());
                }
                if numb == "" {
                    None
                } else {
                    Some(numb)
                }
            })
}

fn find_identifier(chars : &mut Chars) -> Option<String> {
    let mut contains_letter : bool = false;
    let mut ident : String = String::new();
    while match chars.peek() {
        Some(&ch) => {
            if ch.is_alphabetic() {
                contains_letter = true;
            }
            ch.is_alphanumeric() || ch == '_'
        },
        None => false
    } {
        ident.push(chars.next().unwrap());
    }
    if !contains_letter || ident == "" {
        None
    } else {
        Some(ident)
    }
}