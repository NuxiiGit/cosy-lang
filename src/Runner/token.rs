/// An enum which stores token data.
#[allow(dead_code)]
#[derive(Debug)]
pub enum Token {
    Symbol(String),
    Keyword(String),
    Identifier(String),
    Int(String),
    Str(String)
}
impl std::fmt::Display for Token {
    /// Formats the contents of this token.
    #[allow(dead_code)]
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}