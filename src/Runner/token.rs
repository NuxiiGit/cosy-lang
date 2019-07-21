/// A struct which stores token data.
#[allow(dead_code)]
pub struct Token {
    ident : String,
    value : Option<String>
}
impl Token {
    /// Constructs a new token.
    #[allow(dead_code)]
    pub fn new(ident : &str, value : Option<&str>) -> Token {
        let value : Option<String> = if let Some(value) = value {
            Some(value
                    .to_owned()
                    .escape_debug()
                    .collect())
        } else {
            None
        };
        Token {
            ident : ident.to_owned(),
            value : value
        }
    }

    /// Returns the name of this token.
    #[allow(dead_code)]
    pub fn name(&self) -> String {
        self.ident.to_owned()
    }

    /// Returns the value of this token.
    #[allow(dead_code)]
    pub fn value(&self) -> Option<String> {
        if let Some(x) = &self.value {
            Some(x.to_owned())
        } else {
            None
        }
    }
}
impl std::fmt::Display for Token {
    /// Formats the contents of this token.
    #[allow(dead_code)]
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(x) = &self.value {
            write!(f, "Token({}, {})", self.ident, x)
        } else {
            write!(f, "Token({})", self.ident)
        }
    }
}