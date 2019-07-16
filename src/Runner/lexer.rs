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
    identifiers : HashMap<String, String>,

    /// Stores the maps from each pattern to its compiled regexp.
    regexps : HashMap<String, Regex>
}
impl Lexer {
    /// Constructs an instance of `Lexer`.
    #[allow(dead_code)]
    pub fn new() -> Lexer {
        Lexer {
            patterns : Vec::new(),
            identifiers : HashMap::new(),
            regexps : HashMap::new()
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
            if let Ok(regexp) = Regex::new(pattern) {
                self.patterns.push(pattern.to_owned());
                self.regexps.insert(
                    pattern.to_owned(),
                    regexp);
            } else {
                // unable to compile this regexp
                return;
            }
        }
        // update identifier
        self.identifiers.insert(
                pattern.to_owned(),
                ident.to_owned());
    }
    
    /// Tokenises the input expression using this lexer, and returns a `Vec` of tokens `token::Token`.
    /// # Errors
    /// Returns `Err(e)` when the lexer was unable to tokenise this expression.
    #[allow(dead_code)]
    pub fn lex<'a>(&mut self, expression : &str) -> Result<Vec<Token>, &'a str> {

        Err("Not implemented")
    }
}