use super::token::Token;
use super::token::TokenType;

/// A recursive enum which stores expression information.
pub enum Expr {
    Literal(Token),
    Operation(Token, Vec<Expr>)
}