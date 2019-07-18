use std::collections::HashMap;
use super::token::Token;

pub type Chars<'a> = std::iter::Peekable<std::str::Chars<'a>>;
pub type LexMethod = fn(&mut Chars) -> Option<String>;

/// A struct which provides methods for defining and lexing token data.
#[allow(dead_code)]
pub struct Lexer {
    /// Stores the list of token names.
    names : Vec<String>,

    /// Maps from each token name to a closure which returns returns
    /// true or false depending on whether the token could be found 
    /// in some character iterator.
    closures : HashMap<String, LexMethod>,

    /// Stores the closure of tokens to ignore.
    ignore : Option<LexMethod>
}
impl Lexer {
    /// Constructs a new lexer.
    #[allow(dead_code)]
    pub fn new() -> Lexer {
        Lexer {
            names : Vec::new(),
            closures : HashMap::new(),
            ignore : None
        }
    }

    /// Adds a new style to the lexer.
    #[allow(dead_code)]
    pub fn add(&mut self, name : &str, lex : LexMethod) {
        self.names.push(name.to_owned());
        self.closures.insert(name.to_owned(), lex);
    }

    /// Sets the phrase to ignore.
    #[allow(dead_code)]
    pub fn ignore(&mut self, lex : Option<LexMethod>) {
        self.ignore = lex;
    }

    /// Tokenises the input expression into a list of tokens `token::Token`.
    /// # Errors
    /// Returns `None` when the lexer was unable to tokenise this expression.
    #[allow(dead_code)]
    pub fn lex(&mut self, expression :&str) -> Option<Vec<Token>> {
        let mut tokens : Vec<Token> = Vec::new();
        let mut chars : Chars = expression.chars().peekable();
        loop {
            // eliminate ignored phrases
            if let Some(lex) = &self.ignore {
                lex(&mut chars);
            }
            if chars.peek().is_none() {
                // iterator is empty, so there is nothing left to lex
                break;
            }
            // find the longest current token
            let mut best : Option<(String, String, Chars)> = None;
            let mut biggest : usize = 0;
            for name in &self.names {
                if let Some(lex) = &self.closures.get(name) {
                    let mut new_chars : Chars = chars.clone();
                    if let Some(value) = lex(&mut new_chars) {
                        let len : usize = value.chars().count();
                        if len > biggest {
                            best = Some((name.to_owned(), value, new_chars));
                            biggest = len;
                        }
                    }
                }
            }
            // add best to the list of tokens
            if let Some((name, value, remaining_chars)) = best {
                tokens.push(Token::new(&name, &value));
                chars = remaining_chars;
            } else {
                // unexpected symbol
                return None;
            }
        }
        Some(tokens)
    }
}

/// Automatically constructs a closure which matches any whitespace character.
#[macro_export]
macro_rules! lex_whitespace {
    () => (|chars| {
        let mut whitespace : String = String::new();
        while match chars.peek() {
            Some(ch) => ch.is_whitespace(),
            None => false
        } {
            whitespace.push(chars.next().unwrap());
        }
        if whitespace == "" {
            None
        } else {
            Some(whitespace)
        }
    });
}

/// Automatically constructs a closure which matches a keyword.
#[macro_export]
macro_rules! lex_keyword {
    ($keyword : expr) => (|chars| {
        let len : usize = $keyword.chars().count();
        let value : String = chars.take(len).collect();
        if value == $keyword {
            Some(value)
        } else {
            None
        }
    });
}

/// Automatically constructs a closure which matches any phrase 
/// which is between two delimiters.
#[macro_export]
macro_rules! lex_region {
    ($delimiter : expr) => (lex_region!($delimiter, $delimiter));
    ($begin : expr, $end : expr) => (|chars| {
        let len : usize = $begin.chars().count();
        let value : String = chars.take(len).collect();
        if value == $begin {
            // search for $end
            let end : String = $end.chars().rev().collect();
            let size : usize = $end.chars().count();
            let mut inner : String = String::new();
            while let Some(ch) = chars.next() {
                inner.push(ch);
                let delimiter : String = inner
                        .chars()
                        .rev()
                        .take(size)
                        .collect();
                if delimiter == end {
                    let mut i : usize = size;
                    while i > 0 {
                        inner.pop();
                        i -= 1;
                    }
                    return Some(inner);
                }
            }
        }
        None
    });
}

/// Automatically constructs a closure which matches any phrase 
/// which is between some start character and the new line character `\n`.
#[macro_export]
macro_rules! lex_line {
    ($begin : expr) => (|chars| {
        let len : usize = $begin.chars().count();
        let value : String = chars.take(len).collect();
        if value == $begin {
            Some(chars.take_while(|&x| x != '\n').collect())
        } else {
            None
        }
    });
}

