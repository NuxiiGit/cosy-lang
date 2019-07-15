/// A struct which provides functions for defining, lexing, and building the AST of a grammar.
struct Parser {
    
}
impl Parser {
    /// Adds a non-valuable token type.
    fn ignore(pattern : &str) {

    }

    /// Adds a token to the parser grammar.
    fn add(ident : &str, pattern : &str) {
        
    }
    
    /// Tokenises the input expression using this grammar.
    fn lex(expression : &str) -> Vec<Token> {

    }

    /// Parses this `Vec<Token>` of tokens into an abstract syntax tree.
    fn parse(tokens : Vec<Token>) -> SExpression<Token> {

    }
}

/// A struct which stores information about a token.
struct Token {
    
}

/// A recursive enum used to express an abstract syntax tree.
enum SExpression<T> {
    Nil;
    List(T, Vec<SExpression<T>>);
}