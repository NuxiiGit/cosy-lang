use std::{ fmt, error };

use crate::syntax::token::Token;

/// A struct which stores error information.
#[derive(Debug)]
pub struct Error {
    pub reason : &'static str,
    pub token : Token,
    pub kind : ErrorKind
}
impl fmt::Display for Error {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "{}: {}. got {:?}",
                self.token.context, self.reason, self.token.kind)
    }
}
impl error::Error for Error {}

/// An enum which describes available error types.
#[derive(PartialOrd, PartialEq, Debug, Clone)]
pub enum ErrorKind {
    Warning,
    Fatal
}