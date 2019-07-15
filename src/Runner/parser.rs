use runner::token;
use runner::syntax_tree;

/// A struct which provides methods for defining a language grammar, and then using that to construct an abstract syntax tree.
pub struct Parser {
    
}
impl Parser {
    /// Parses this `Vec<Token>` of tokens into an abstract syntax tree.
    pub fn parse(tokens : Vec<Token>) -> SExpression<Token> {
        SExpression::Nil
    }
}