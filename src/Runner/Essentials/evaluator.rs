#![allow(dead_code)]

use super::syntax_tree::SyntaxTree;

/// A trait which describes an evaluator type.
pub trait Evaluator<'a, T> {
    /// Evaluates this parse tree into some return value.
    fn run(tree : SyntaxTree<'a>) -> T;
}

/// An enum which describes the different types of value.
pub enum Values {
    Char(char),
    Integer(i64),
    Natural(u64),
    Float(f64)
}