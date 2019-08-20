/// An enum which stores token data.
#[allow(dead_code)]
#[derive(Debug)]
pub enum Token {
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
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    SemiColon,
    // Operators
    Plus,
    Dash,
    Star,
    ForwardSlash,
    BackwardSlash
}
impl std::fmt::Display for Token {
    /// Formats the contents of this token.
    #[allow(dead_code)]
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}