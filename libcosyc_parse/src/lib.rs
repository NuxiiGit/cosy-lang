pub mod syntax;

//use libcosyc_diagnostic::{
//    source::Span,
//    error::{ IssueTracker, CompilerError, ErrorLevel }
//};
use libcosyc_scan::{ Lexer, token::TokenKind };
//use crate::syntax as ast;
//use std::mem;

/// Produces a concrete syntax tree from concrete syntax.
pub struct Parser<'a> {
    _lexer : Lexer<'a>,
    _peeked : TokenKind
}
