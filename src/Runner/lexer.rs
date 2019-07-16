extern crate regex;

use regex::Regex;
use super::token::Token;

/// A struct which which provides methods for defining and lexing token data.
pub struct Lexer {
    
}
impl Lexer {
    /// Adds a non-valuable token type.
    pub fn ignore(pattern : &str) {

    }

    /// Adds a token to the parser grammar.
    pub fn add(ident : &str, pattern : &str) {
        
    }
    
    /// Tokenises the input expression using this grammar.
    pub fn lex(expression : &str) -> Vec<Token> {
        Vec::new()
    }
}