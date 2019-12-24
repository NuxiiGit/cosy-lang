use crate::diagnostics::*;
use crate::syntax::token::*;

use std::str::CharIndices;

/// An iterator over a string slice, which produces `Token`s.
pub struct Lexer<'a> {
    scanner : StringScanner<'a>,
    eof : bool
}
impl<'a> Lexer<'a> {
    /// Creates a new lexer from this string scanner.
    pub fn from(scanner : StringScanner<'a>) -> Self {
        Lexer {
            scanner,
            eof : false
        }
    }
}
impl<'a> Iterator for Lexer<'a> {
    type Item = Result<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.scanner.ignore();
        let result = if let Some(x) = self.scanner.advance() {
            match x {
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
                    self.scanner.advance();
                    let documentation = Some('|') == self.scanner.chr();
                    while let Some(x) = self.scanner.chr() {
                        if let '\n' | '\r' = x {
                            break;
                        } else {
                            self.scanner.advance();
                        }
                    }
                    if documentation {
                        Ok(TokenKind::Documentation)
                    } else {
                        return self.next();
                    }
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
                '(' => {
                    if let Some(')') = self.scanner.chr() {
                        self.scanner.advance();
                        Ok(TokenKind::Identifier(IdentifierKind::Empty))
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
                ':' => {
                    if let Some(':') = self.scanner.chr() {
                        self.scanner.advance();
                        Ok(TokenKind::ColonColon)
                    } else {
                        Ok(TokenKind::Colon)
                    }
                },
                ';' => Ok(TokenKind::SemiColon),
                '`' => Ok(TokenKind::Backtick),
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
                        "\\" => Ok(TokenKind::Backslash),
                        _ => Ok(TokenKind::Identifier(IdentifierKind::Operator))
                    }
                },
                // match number literals
                x if valid_digit(x) => {
                    let mut is_real = false;
                    while let Some(x) = self.scanner.chr() {
                        if x == '.' {
                            if is_real {
                                break;
                            } else {
                                is_real = true;
                            }
                        } else if !valid_digit(x) {
                            break;
                        }
                        self.scanner.advance();
                    }
                    Ok(TokenKind::Literal(if is_real {
                        LiteralKind::Real
                    } else {
                        LiteralKind::Integer
                    }))
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
                        "is" => Ok(TokenKind::Is),
                        "while" => Ok(TokenKind::While),
                        "until" => Ok(TokenKind::Until),
                        "repeat" => Ok(TokenKind::Repeat),
                        "for" => Ok(TokenKind::For),
                        "in" => Ok(TokenKind::In),
                        "function" => Ok(TokenKind::Function),
                        "object" => Ok(TokenKind::Object),
                        "new" => Ok(TokenKind::New),
                        _ => Ok(TokenKind::Identifier(IdentifierKind::Alphanumeric))
                    }
                },
                // unknown lex
                _ => Err("unknown symbol")
            }
        } else if self.eof {
            return None;
        } else {
            self.eof = true;
            Ok(TokenKind::EoF)
        };
        let span = self.scanner.span();
        Some(match result {
            Ok(kind) => Ok(Token { kind, span }),
            Err(reason) => {
                let token = Token { kind : TokenKind::Unknown, span };
                Err(Error { reason, token })
            }
        })
    }
}

/// The result of the lexer.
pub type Result<'a> = std::result::Result<Token<'a>, Error<'a>>;

/// A function which returns whether this character is a valid operator character.
pub fn valid_operator(x : char) -> bool {
    if let '!' | '?' |
            '@' | '$' | '&' | '#' |
            '+' | '-' | '*' | '/' | '\\' | '%' | '^' |
            '<' | '=' | '>' |
            '|' | '~' = x {
        true
    } else {
        !(x.is_ascii() || valid_graphic(x) || valid_whitespace(x))
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
pub struct StringScanner<'a> {
    context : &'a str,
    chars : CharIndices<'a>,
    peeked : Option<char>,
    row : (usize, usize),
    column : (usize, usize),
    span_begin : usize,
    span_end : usize,
}
impl<'a> StringScanner<'a> {
    /// Create a new scanner from this string slice.
    pub fn from(context : &'a str) -> StringScanner<'a> {
        let mut chars = context.char_indices();
        let peeked = if let Some((_, x)) = chars.next() {
            // get the first character
            // this allows for the string scanner to have an immutable `chr` method
            Some(x)
        } else {
            None
        };
        StringScanner {
            context,
            chars,
            peeked,
            row : (1, 1),
            column : (1, 1),
            span_begin : 0,
            span_end : 0,
        }
    }

    /// Peeks at the current substring.
    pub fn substr(&self) -> &'a str {
        &self.context[self.span_begin..self.span_end]
    }

    /// Erases the current substring.
    pub fn ignore(&mut self) {
        self.span_begin = self.span_end;
        self.row.0 = self.row.1;
        self.column.0 = self.column.1;
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
                self.row.1 += 1;
                self.column.1 = 0;
            } else {
                self.column.1 += 1;
            }
            Some(x)
        } else {
            // end of file
            self.span_end = self.context.len();
            None
        };
        previous
    }

    /// Return the curent span.
    pub fn span(&self) -> Span<'a> {
        Span {
            content : self.substr(),
            row : self.row.0,
            column : self.column.0
        }
    }
}