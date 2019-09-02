/// A recursive enum which stores expression information.
#[allow(dead_code)]
#[derive(Debug)]
pub enum Expr {
    // literals
    Int(String),
    Str(String),
    Identifier(String),
    // operators
    Unary(Box<Expr>),
    Binary(Box<Expr>)
}
impl std::fmt::Display for Expr {
    /// Formats the contents of this `Expr`.
    #[allow(dead_code)]
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}