use super::interface::lexer::Chars;
use super::interface::lexer::Lexer;

/// Generates the language specific lexer.
pub fn generate_lexer() -> Lexer {
    Lexer::new()
            .ignore(Some(lex_whitespace!()))
            // comments
            .add("COMMENT", lex_line!("''"))
            .add("COMMENT_MULTILINE", lex_region!("'{", "}'"))
            // blocks
            .add("BRACKET_BEGIN", lex_keyword!("("))
            .add("BRACKET_END", lex_keyword!(")"))
            .add("BLOCK_BEGIN", lex_keyword!("{"))
            .add("BLOCK_END", lex_keyword!("}"))
            .add("ENDLINE", lex_keyword!(";"))
            // keywords
            .add("VAR", lex_keyword!("var"))
            .add("IF", lex_keyword!("if"))
            .add("IFNOT", lex_keyword!("ifnot"))
            .add("ELSE", lex_keyword!("else"))
            .add("REPEAT", lex_keyword!("repeat"))
            .add("WHILE", lex_keyword!("while"))
            .add("UNTIL", lex_keyword!("until"))
            .add("FOR", lex_keyword!("for"))
            .add("FUNCTION", lex_keyword!("function"))
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