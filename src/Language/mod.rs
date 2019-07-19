use super::runner::lexer::Chars;
use super::runner::lexer::Lexer;

/// Generates the language specific lexer.
pub fn generate_lexer() -> Lexer {
    let mut lexer : Lexer = Lexer::new();
    // whitespace
    lexer.ignore(Some(lex_whitespace!()));
    // comments
    lexer.add("COMMENT", lex_line!("''"));
    lexer.add("COMMENT_MULTILINE", lex_region!("'{", "}'"));
    // blocks
    lexer.add("BRACKET_BEGIN", lex_keyword!("("));
    lexer.add("BRACKET_END", lex_keyword!(")"));
    lexer.add("BLOCK_BEGIN", lex_keyword!("{"));
    lexer.add("BLOCK_END", lex_keyword!("}"));
    lexer.add("ENDLINE", lex_keyword!(";"));
    // keywords
    lexer.add("VAR", lex_keyword!("var"));
    lexer.add("IF", lex_keyword!("if"));
    lexer.add("IFNOT", lex_keyword!("ifnot"));
    lexer.add("ELSE", lex_keyword!("else"));
    lexer.add("REPEAT", lex_keyword!("repeat"));
    lexer.add("WHILE", lex_keyword!("while"));
    lexer.add("UNTIL", lex_keyword!("until"));
    lexer.add("FOR", lex_keyword!("for"));
    lexer.add("FUNCTION", lex_keyword!("function"));
    // identifiers
    lexer.add("IDENTIFIER", |chars| find_identifier(chars));
    lexer.add("LABEL", |chars| {
        if let Some('\'') = chars.next() {
            find_identifier(chars)
        } else {
            None
        }
    });
    lexer.add("STRING", lex_region!("\""));
    lexer.add("NUMBER", |chars| {
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
    });
    lexer
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