use crate::lexer::{ Lexer, TokenKind, LiteralKind, IdentifierKind };
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
	},
	NoOp
}

/// Represents expression information.
#[derive(Debug, Clone)]
pub enum Expr {
	Variable,
	Value {
		kind : ValueKind
	}
}

/// Represents the different primitive variants.
#[derive(Debug, Clone)]
pub enum ValueKind {
	Integer
}

/// Represents metadata of a AST node.
#[derive(Debug, Clone)]
pub struct Node<T : fmt::Debug + Clone> {
	pub content : T,
	pub span : Span
}