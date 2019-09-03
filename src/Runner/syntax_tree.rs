#![allow(dead_code)]

use std::fmt;

/// A recursive enum which stores expression information.
#[derive(Debug)]
pub enum Expr<'a> {
    // literals
    Integer(&'a str),
    String(&'a str),
    Variable(&'a str),
    // operators
    Unary(&'a str, Box<Expr<'a>>),
    Binary(&'a str, Box<Expr<'a>>, Box<Expr<'a>>)
}
impl<'a> fmt::Display for Expr<'a> {
    /// Formats the contents of this `Expr`.
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}