/// A struct which stores location information about a `TokenType`.
#[allow(dead_code)]
pub struct Token {
    flavour : TokenType,
    line : usize,
    column : usize
}
impl Token {
    /// Creates a new instance of `Token`.
    #[allow(dead_code)]
    pub fn new(flavour : TokenType, line : usize, column : usize) -> Token {
        Token {
            flavour : flavour,
            line : line,
            column : column
        }
    }

    /// Returns a reference to the type of this `Token`.
    #[allow(dead_code)]
    pub fn flavour(&self) -> &TokenType {
        &self.flavour
    }

    /// Returns the line of this `Token`.
    #[allow(dead_code)]
    pub fn line(&self) -> usize {
        self.line.to_owned()
    }

    /// Returns the column of this `Token`.
    #[allow(dead_code)]
    pub fn column(&self) -> usize {
        self.column.to_owned()
    }
}
impl std::fmt::Display for Token {
    /// Formats the contents of this `Token`.
    #[allow(dead_code)]
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.flavour)
    }
}

/// An enum which stores the type of `Token`.
#[allow(dead_code)]
#[derive(Debug)]
pub enum TokenType {
    // literals
    Str(String),
    Int(String),
    // Keywords
    Identifier(String),
    Var,
    If,
    IfNot,
    Else,
    // Symbols
    Operator(String),
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    SemiColon
}
impl std::fmt::Display for TokenType {
    /// Formats the contents of this `TokenType`.
    #[allow(dead_code)]
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}