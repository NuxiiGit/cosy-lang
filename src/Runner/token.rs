/// A struct which stores information about a token.
pub struct Token {
    pub ident : String,
    pub value : String,
    pub line : usize,
    pub column : usize
}