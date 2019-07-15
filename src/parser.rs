extern crate regex;
use regex::Regex;

/// A struct which provides functions for defining, lexing, and building the AST of a grammar.
pub struct Parser {
    
}
impl Parser {
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

    /// Parses this `Vec<Token>` of tokens into an abstract syntax tree.
    pub fn parse(tokens : Vec<Token>) -> SExpression<Token> {
        SExpression::Nil
    }
}

/// A struct which stores information about a token.
pub struct Token {
    
}

/// A recursive enum used to express an abstract syntax tree.
pub enum SExpression<T> {
    Nil,
    List(T, Vec<SExpression<T>>)
}