extern crate regex;

use std::collections::hash_map::HashMap;
use regex::Regex;
use super::token::Token;

/// Stores the keyword for the ignore token.
#[allow(dead_code)]
const IGNORE : &str = "WHITESPACE";

/// A struct which which provides methods for defining and lexing token data.
#[allow(dead_code)]
pub struct Lexer {
    /// Stores the available patterns in order of preceedence.
    patterns : Vec<String>,

    /// Stores the maps from each pattern to its token identifier.
    /// This constrains each token type to a single pattern.
    identifiers : HashMap<String, String>
}
impl Lexer {
    /// Constructs an instance of `Lexer`.
    #[allow(dead_code)]
    pub fn new() -> Lexer {
        Lexer {
            patterns : Vec::new(),
            identifiers : HashMap::new()
        }
    }

    /// Adds a non-valuable token type.
    #[allow(dead_code)]
    pub fn ignore(&mut self, pattern : &str) {
        self.add(IGNORE, pattern);
    }

    /// Adds a token to the parser grammar.
    #[allow(dead_code)]
    pub fn add(&mut self, ident : &str, pattern : &str) {
        if let None = self.patterns
                .iter()
                .position(|x| x == pattern) {
            // add new pattern
            self.patterns.push(pattern.to_owned());
        }
        // update identifier
        self.identifiers.insert(
                pattern.to_owned(),
                ident.to_owned());
    }
    
    /// Tokenises the input expression using this grammar.
    #[allow(dead_code)]
    pub fn lex(&mut self, expression : &str) -> Vec<Token> {
        Vec::new()
    }
}