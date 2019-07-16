extern crate regex;

use std::fmt;
use std::collections::hash_map::HashMap;
use regex::Regex;
use super::token::Token;

/// Stores the keyword for the ignore token.
#[allow(dead_code)]
const IGNORE : &str = "WHITESPACE";

/// A struct which which provides methods for defining and lexing token data.
#[allow(dead_code)]
pub struct Lexer {
    /// Stores the available patterns in no required order.
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
    /// # Current Implementation
    /// The idea behind tokenising is to find the "best-fit", left-most token which 
    #[allow(dead_code)]
    pub fn lex<'a>(&mut self, expression : &str) -> Result<Vec<Token>, &'a str> {
        
        Err("Not implemented")
    }

    /// Finds the "best-fit", left-most token in this expression, then returns the start and end positions of this substring.
    /// # Errors
    /// Returns `None` when no valid token was found.
    pub fn find_best_fit(&mut self, expression : &str, start : usize) -> Option<(String, usize, usize)> {
        let mut name : String = String::new();
        let mut left : usize = std::usize::MAX;
        let mut right : usize = std::usize::MIN;
        for pattern in &self.patterns {
            let ident : &str = self.identifiers.get(pattern).unwrap();
            let regexp : &Regex = self.regexps.get(pattern).unwrap();
            if let Some(pos) = regexp.find_at(expression, start) {
                let start : usize = pos.start();
                let end : usize = pos.end();
                if (start < left)
                || (start == left && end > right) {
                    // update record
                    name = ident.to_owned();
                    left = start;
                    right = end;
                }
            }
        }
        if left < right {
            Some((name, left, right))
        } else {
            None
        }
    }
}
impl fmt::Display for Lexer {
    /// Formats the contents of this lexer.
    #[allow(dead_code)]
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        let mut msg : String = String::new();
        for pattern in &self.patterns {
            let ident : &str = self.identifiers.get(pattern).unwrap();
            if &msg != "" {
                msg.push_str(", ");
            }
            msg.push_str(&format!("({}, {})", ident, pattern));
        }
        write!(f, "[{}]", msg)
    }
}