pub mod collections;
pub mod lexer;
pub mod lexer_improved;
pub mod parser;
pub mod parser_improved;
pub mod interpreter;

/// A type which represents a boxed error type.
pub type RunnerError = Box<std::error::Error>;

/// A type which defines the main result of the runner.
pub type Result<T> = std::result::Result<T, RunnerError>;