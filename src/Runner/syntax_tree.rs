/// A recursive enum which stores expression information.
#[allow(dead_code)]
#[derive(Debug)]
pub enum Expr {
    Int(String),
    Str(String),
    Identifier(String),
    Operation(Operator, Vec<Expr>)
}
impl std::fmt::Display for Expr {
    /// Formats the contents of this `Expr`.
    #[allow(dead_code)]
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expr::Int(_) => write!(f, "{:?}", self),
            _ => write!(f, "{:#?}", self)
        }
    }
}

/// An enum which stores operator information.
#[allow(dead_code)]
#[derive(Debug)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide
}
impl std::fmt::Display for Operator {
    /// Formats the contents of this `Operator`.
    #[allow(dead_code)]
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}