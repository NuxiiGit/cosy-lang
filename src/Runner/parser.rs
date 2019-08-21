use super::token::*;
use super::syntax_tree::*;

/// A type which represents the char iterator used by the lexer.
#[allow(dead_code)]
type Tokens<'a> = std::iter::Peekable<std::slice::Iter<'a, Token>>;

/// Parses an array of `Token`s into a parse tree.
#[allow(dead_code)]
pub fn parse(tokens : &[Token]) -> Result<Expr, &'static str> {
    let mut tokens : Tokens = tokens
            .iter()
            .peekable();
    expression(&mut tokens)
}

fn expression(tokens : &mut Tokens) -> Result<Expr, &'static str> {
    addition(tokens)
}

fn addition(tokens : &mut Tokens) -> Result<Expr, &'static str> {
    let mut expr : Expr = multiplication(tokens)?;
    while let Some(token) = tokens.peek() {
        let flavour : &TokenType = token.flavour();
        if let TokenType::Plus | TokenType::Minus = flavour {
            tokens.next();
            let right : Expr = multiplication(tokens)?;
            expr = match flavour {
                TokenType::Plus => Expr::Add(Box::new(expr), Box::new(right)),
                TokenType::Minus => Expr::Sub(Box::new(expr), Box::new(right)),
                _ => unreachable!()
            }
        } else {
            break;
        }
    }
    Ok(expr)
}

fn multiplication(tokens : &mut Tokens) -> Result<Expr, &'static str> {
    let mut expr : Expr = primary(tokens)?;
    while let Some(token) = tokens.peek() {
        let flavour : &TokenType = token.flavour();
        if let TokenType::Star | TokenType::ForwardSlash = flavour {
            tokens.next();
            let right : Expr = primary(tokens)?;
            expr = match flavour {
                TokenType::Star => Expr::Multiply(Box::new(expr), Box::new(right)),
                TokenType::ForwardSlash => Expr::Divide(Box::new(expr), Box::new(right)),
                _ => unreachable!()
            }
        } else {
            break;
        }
    }
    Ok(expr)
}

fn primary(tokens : &mut Tokens) -> Result<Expr, &'static str> {
    if let Some(token) = tokens.next() {
        match token.flavour() {
            TokenType::Str(x) => Ok(Expr::Str(x.to_owned())),
            TokenType::Int(x) => Ok(Expr::Int(x.to_owned())),
            TokenType::Identifier(x) => Ok(Expr::Identifier(x.to_owned())),
            TokenType::LeftParen => {
                let expr : Expr = expression(tokens)?;
                match tokens.next() {
                    Some(token) if match token.flavour() {
                        TokenType::RightParen => true,
                        _ => false
                    } => Ok(expr),
                    _ => Err("Expected ')' after expression")
                }
            },
            _ => Err("Expected identifier or literal")
        }
    } else {
        Err("Expected expression: Got nothing")
    }
}

/*
 * expression      -> addition
 * addition        -> multiplication (("-" | "+") multiplication)*
 * multiplication  -> unary (("*" | "/") unary)*
 * unary           -> ("-" | "+")* primary
 * primary         -> INT | "(" expression ")"
 */