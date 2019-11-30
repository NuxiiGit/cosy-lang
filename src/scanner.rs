use super::source_pos::Span;
use super::error::Error;
use super::token::{
    Token,
    TokenKind,
    IdentifierKind,
    LiteralKind
};

use std::str::CharIndices;

/// An iterator over a string slice, which produces `Token`s.
pub struct Lexer<'a> {
    scanner : StrScanner<'a>
}
impl<'a> Lexer<'a> {
    /// Create a new lexer.
    pub fn lex(scanner : StrScanner<'a>) -> Lexer<'a> {
        Lexer { scanner }
    }
}
impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, Error<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.scanner.ignore();
        let row = self.scanner.row();
        let column = self.scanner.column();
        let result = match self.scanner.advance()? {
            // ignore whitespace
            x if valid_whitespace(x) => {
                while let Some(x) = self.scanner.chr() {
                    if !valid_whitespace(x) {
                        break;
                    }
                    self.scanner.advance();
                }
                return self.next();
            },
            // ignore line comment
            '/' if Some('/') == self.scanner.chr() => {
                while let Some(x) = self.scanner.advance() {
                    if x == '\n' {
                        break;
                    }
                }
                return self.next()
            },
            // ignore block comments
            '/' if Some('*') == self.scanner.chr() => {
                let mut nests = 1;
                while let Some(x) = self.scanner.advance() {
                    match x {
                        '*' if Some('/') == self.scanner.chr() => {
                            if nests == 1 {
                                self.scanner.advance();
                                return self.next();
                            } else {
                                nests -= 1;
                            }
                        },
                        '/' if Some('*') == self.scanner.chr() => {
                            nests += 1;
                        },
                        _ => continue
                    }
                }
                Err("unterminated block comment")
            }
            // match special symbols
            x if valid_symbol(x) => {
                match x {
                    '(' => {
                        if let Some(')') = self.scanner.chr() {
                            self.scanner.advance();
                            Ok(TokenKind::Empty)
                        } else {
                            Ok(TokenKind::LeftParen)
                        }
                    },
                    ')' => Ok(TokenKind::RightParen),
                    '{' => Ok(TokenKind::LeftBrace),
                    '}' => Ok(TokenKind::RightBrace),
                    '[' => Ok(TokenKind::LeftBox),
                    ']' => Ok(TokenKind::RightBox),
                    '.' => Ok(TokenKind::Dot),
                    ',' => Ok(TokenKind::Comma),
                    ':' => Ok(TokenKind::Comma),
                    ';' => Ok(TokenKind::SemiColon),
                    '"' => {
                        // get string literal
                        loop {
                            if let Some(x) = self.scanner.advance() {
                                if x == '\\' {
                                    self.scanner.advance();
                                } else if x == '"' {
                                    break Ok(TokenKind::Literal(LiteralKind::String));
                                }
                            } else {
                                break Err("unterminated string literal");
                            }
                        }
                    },
                    '\'' => {
                        // get char literal
                        loop {
                            if let Some(x) = self.scanner.advance() {
                                if x == '\\' {
                                    self.scanner.advance();
                                } else if x == '\'' {
                                    break Ok(TokenKind::Literal(LiteralKind::Character));
                                }
                            } else {
                                break Err("unterminated character literal");
                            }
                        }
                    },
                    '`' => {
                        // get identifier literal
                        loop {
                            if let Some(x) = self.scanner.advance() {
                                if x == '`' {
                                    break Ok(TokenKind::Identifier(IdentifierKind::Literal));
                                }
                            } else {
                                break Err("unterminated identifier literal");
                            }
                        }
                    },
                    _ => Err("unknown reserved symbol")
                }
            },
            // match operators
            x if valid_operator(x) => {
                while let Some(x) = self.scanner.chr() {
                    if !valid_operator(x) {
                        break;
                    }
                    self.scanner.advance();
                }
                match self.scanner.substr() {
                    "->" => Ok(TokenKind::Arrow),
                    "=" => Ok(TokenKind::Assign),
                    _ => Ok(TokenKind::Identifier(IdentifierKind::Operator))
                }
            },
            // match number literals
            x if valid_digit(x) => {
                while let Some(x) = self.scanner.chr() {
                    if !valid_digit(x) {
                        break;
                    }
                    self.scanner.advance();
                }
                Ok(TokenKind::Literal(LiteralKind::Integer))
            },
            // match keywords and identifiers
            x if valid_graphic(x) => {
                while let Some(x) = self.scanner.chr() {
                    if !valid_graphic(x) {
                        break;
                    }
                    self.scanner.advance();
                }
                match self.scanner.substr() {
                    "var" => Ok(TokenKind::Var),
                    "const" => Ok(TokenKind::Const),
                    "if" => Ok(TokenKind::If),
                    "unless" => Ok(TokenKind::Unless),
                    "else" => Ok(TokenKind::Else),
                    "then" => Ok(TokenKind::Then),
                    "switch" => Ok(TokenKind::Switch),
                    "case" => Ok(TokenKind::Case),
                    "while" => Ok(TokenKind::While),
                    "until" => Ok(TokenKind::Until),
                    "repeat" => Ok(TokenKind::Repeat),
                    "function" => Ok(TokenKind::Function),
                    "object" => Ok(TokenKind::Object),
                    "new" => Ok(TokenKind::New),
                    "trait" => Ok(TokenKind::Trait),
                    "instance" => Ok(TokenKind::Instance),
                    _ => Ok(TokenKind::Identifier(IdentifierKind::Alphanumeric))
                }
            },
            // unknown lex
            _ => Err("unknown character")
        };
        let span = Span { content : self.scanner.substr(), row, column };
        Some(match result {
            Ok(kind) => Ok(Token { kind, span }),
            Err(reason) => Err(Error { reason, span })
        })
    }
}

/// A function which returns whether this character is a valid operator character.
pub fn valid_operator(x : char) -> bool {
    !(valid_symbol(x) || valid_whitespace(x) || valid_graphic(x))
}

/// A function which returns whether this character is a valid symbol character.
pub fn valid_symbol(x : char) -> bool {
    if let '(' | ')' | '{' | '}' | '[' | ']' |
            '.' | ',' | ':' | ';' |
            '\'' | '"' | '`' = x {
        true
    } else {
        false
    }
}

/// A function which returns whether this character is a valid whitespace character.
pub fn valid_whitespace(x : char) -> bool {
    x.is_whitespace()
}

/// A function which returns whether this character is a valid identifier character.
pub fn valid_graphic(x : char) -> bool {
    x == '\'' || x == '_' || x.is_alphanumeric()
}

/// A function which returns whether this character is a valid number character.
pub fn valid_digit(x : char) -> bool {
    x.is_ascii_digit()
}

/// A structure over a string slice which produces individual `Span`s of tokens.
pub struct StrScanner<'a> {
    context : &'a str,
    chars : CharIndices<'a>,
    peeked : Option<char>,
    row : usize,
    column : usize,
    span_begin : usize,
    span_end : usize
}
impl<'a> StrScanner<'a> {
    /// Create a new scanner from this string slice.
    pub fn from(context : &'a str) -> StrScanner<'a> {
        let mut chars = context.char_indices();
        let peeked = if let Some((_, x)) = chars.next() {
            // get the first character
            // this allows for the string scanner to have an immutable `chr` method
            Some(x)
        } else {
            None
        };
        StrScanner {
            context,
            chars,
            peeked,
            row : 1,
            column : 1,
            span_begin : 0,
            span_end : 0,
        }
    }

    /// Returns the current column of the scanner.
    pub fn column(&self) -> usize {
        self.column
    }

    /// Returns the current row of the scanner.
    pub fn row(&self) -> usize {
        self.row
    }

    /// Peeks at the current substring.
    pub fn substr(&mut self) -> &'a str {
        &self.context[self.span_begin..self.span_end]
    }

    /// Erases the current substring.
    pub fn ignore(&mut self) {
        self.span_begin = self.span_end;
    }

    /// Peek at the next character.
    pub fn chr(&self) -> Option<char> {
        self.peeked
    }

    /// Move to the next character.
    pub fn advance(&mut self) -> Option<char> {
        let previous = self.chr();
        self.peeked = if let Some((i, x)) = self.chars.next() {
            // update span
            self.span_end = i;
            // move to new line
            if x == '\n' {
                self.row += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            Some(x)
        } else {
            // end of file
            self.span_end = self.context.len();
            None
        };
        previous
    }
}