#![allow(dead_code)]

use super::syntax_tree::SyntaxTree;

/// A trait which describes an evaluator type.
pub trait Evaluator<'a, T> {
    /// Evaluates this parse tree into some return value.
    fn run(&mut self, tree : &SyntaxTree<'a>) -> Option<T>;
}

/// An enum which describes the different types of value.
pub enum Value {
    Char(char),
    Integer(i64),
    Float(f64)
}