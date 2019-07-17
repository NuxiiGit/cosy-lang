use std::fmt;
use super::token::Token;

/// A struct which which provides methods for defining and lexing token data.
#[allow(dead_code)]
pub struct Lexer<T> {
    /// Stores the list of objects which implement the `LexType` trait beside their identifier.
    identifiers : Vec<(String, T)>,

    /// Stores a single identifier which determines what phrases to ignore.
    ignore : Option<T>
}
impl<T> Lexer<T> where
        T : LexType {
    /// Constructs a new instance of `Lexer<T>`.
    #[allow(dead_code)]
    pub fn new() -> Lexer<T> {
        Lexer {
            identifiers : Vec::new(),
            ignore : None
        }
    }

    /// Adds a new record to the lexer.
    #[allow(dead_code)]
    pub fn add(&mut self, ident : &str, lex : T) {
        self.identifiers.push((ident.to_owned(), lex));
    }

    /// Sets the phrase to ignore
    #[allow(dead_code)]
    pub fn ignore(&mut self, lex : Option<T>) {
        self.ignore = lex;
    }

    /// Tokenises the input expression using this lexer, and returns a `Vec` of tokens `token::Token`.
    /// # Errors
    /// Returns `None` when the lexer was unable to tokenise this expression.
    #[allow(dead_code)]
    pub fn lex<'a>(&mut self, expression : &str) -> Option<Vec<Token>> {
        let mut tokens : Vec<Token> = Vec::new();
        let mut start : usize = 0;
        let size : usize = expression.chars().count();
        while start < size {
            // eliminate ignored phrases
            if let Some(lex_type) = &self.ignore {
                if let Some(end) = lex_type.find(expression, start) {
                    if end > start {
                        start = end;
                    }
                }
            }
            // find the longest current token
            let mut best_end : usize = start;
            let mut best_ident : Option<String> = None;
            for (ident, lex_type) in &self.identifiers {
                if let Some(end) = lex_type.find(expression, start) {
                    if end > best_end {
                        best_end = end;
                        best_ident = Some(ident.to_owned());
                    }
                }
            }
            if let Some(ident) = best_ident {
                if start < best_end {
                    let value : String = expression
                            .chars()
                            .skip(start)
                            .take(best_end - start)
                            .collect();
                    tokens.push(Token {
                        ident : ident,
                        value : value,
                        line : 1,
                        column : 1
                    });
                } else {
                    // unexpected symbol
                    return None;
                }
            }
            start = best_end;
        }
        Some(tokens)
    }
}
impl<T> fmt::Display for Lexer<T> {
    /// Formats the contents of this lexer.
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        let mut msg : String = String::new();
        for (ident, _) in &self.identifiers {
            if !msg.is_empty() {
                msg.push_str(", ");
            }
            msg.push_str(&ident);
        }
        write!(f, "({})", msg)
    }
}

/// A trait which lets the lexer have exotic behaviour defined by the language developer.
pub trait LexType {
    /// Return the end position of a valid substring for this lex type.
    fn find(&self, s : &str, start : usize) -> Option<usize>;
}