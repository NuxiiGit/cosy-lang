use std::fmt;

/// A struct which stores information about a token.
pub struct Token {
    pub ident : String,
    pub value : String,
    pub line : usize,
    pub column : usize
}
impl fmt::Display for Token {
    /// Formats the contents of this token.
    #[allow(dead_code)]
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ident={}, value={}, line={}, column={}]",
                self.ident,
                self.value,
                self.line,
                self.column)
    }
}