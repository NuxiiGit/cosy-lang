pub mod syntax_tree;
pub mod error;

/// A type alias for script locations. Read as `(row, column)`.
pub type Position = (usize, usize);