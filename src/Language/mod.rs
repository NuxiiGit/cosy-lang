use super::runner::lexer::Lexer;

/// Generates the language specific lexer.
pub fn generate_lexer() -> Lexer {
    let mut lexer : Lexer = Lexer::new();
    // whitespace
    lexer.ignore(Some(lex_whitespace!()));
    // comments
    lexer.add("COMMENT", lex_line!("''"));
    lexer.add("COMMENT_MULTILINE", lex_region!("'{", "}'"));
    // keywords
    lexer.add("IF", lex_keyword!("if"));
    lexer.add("IFNOT", lex_keyword!("ifnot"));
    lexer
}