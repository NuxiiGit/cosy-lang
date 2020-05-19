pub mod lexer;

use crate::span::Span;

use std::fmt;

/// Represents information about the program.
#[derive(Debug)]
pub struct Program {
	pub body : Block
}

/// Represents a block of statements
#[derive(Debug)]
pub struct Block {
	pub stmts : Vec<Node<Stmt>>
}

/// Represents statement information.
#[derive(Debug, Clone)]
pub enum Stmt {
	Decl,
	Expr {
		expr : Node<Expr>
	}
}

/// Represents expression information.
#[derive(Debug, Clone)]
pub enum Expr {
	Variable,
	Value {
		kind : ValueKind
	},
	NoOp
}

/// Represents the different primitive variants.
#[derive(Debug, Clone)]
pub enum ValueKind {
	Integer
}

/// Represents metadata of a AST node.
#[derive(Clone)]
pub struct Node<T : fmt::Debug + Clone> {
	pub content : T,
	pub span : Span
}
impl<T : fmt::Debug + Clone> fmt::Debug for Node<T> {
	fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
		write!(out, "({}){:?}", self.span, self.content)
	}
}