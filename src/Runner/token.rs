/// A struct which stores token data.
#[allow(dead_code)]
pub struct Token {
    name : String,
    value : String
}
impl Token {
    /// Constructs a new token.
    #[allow(dead_code)]
    pub fn new(name : &str, value : &str) -> Token {
        Token {
            name : name.to_owned(),
            value : value.to_owned()
        }
    }
}
impl std::fmt::Display for Token {
    /// Formats the contents of this token.
    #[allow(dead_code)]
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})",
                self.name,
                self.value
                        .replace("\n", r"\n")
                        .replace("\t", r"\t")
                        .replace("\r", r"\r"))
    }
}