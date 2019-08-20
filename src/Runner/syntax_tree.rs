/// A recursive enum which stores expression information.
#[allow(dead_code)]
pub enum Expr {
    Terminal(String),
    Operation(Operator, Vec<Expr>)
}

/// An enum which stores operator information.
#[allow(dead_code)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide
}