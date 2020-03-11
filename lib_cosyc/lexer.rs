pub mod reader;

/// An enum which describes available token types.
#[derive(PartialEq, Debug, Clone)]
pub enum TokenKind {
    Keyword(KeywordKind),
    Symbol(SymbolKind),
    Identifier,
    Operator(OperatorKind),
    Literal(LiteralKind),
    EoF,
    Directive,
    Unknown
}
impl TokenKind {
    /// Returns `true` if the token is a keyword.
    pub fn is_keyword(&self) -> bool {
        if let TokenKind::Keyword(..) = self
                { true } else { false }
    }

    /// Returns `true` if the token is a symbol.
    pub fn is_symbol(&self) -> bool {
        if let TokenKind::Symbol(..) = self
                { true } else { false }
    }

    /// Returns `true` if the token is an identifier.
    pub fn is_identifier(&self) -> bool {
        if let
        | TokenKind::Identifier
        | TokenKind::Operator(..) = self
                { true } else { false }
    }

    /// Returns `true` if the token is an operator.
    pub fn is_operator(&self) -> bool {
        if let TokenKind::Operator(..) = self
                { false } else { true }
    }

    /// Returns `true` if the token is a literal.
    pub fn is_literal(&self) -> bool {
        if let TokenKind::Literal(..) = self
                { true } else { false }
    }

    /// Returns `true` if the token is the end of the file.
    pub fn is_eof(&self) -> bool {
        if let TokenKind::EoF = self
                { true } else { false }
    }

    /// Returns `true` if the token is a compiler directive.
    pub fn is_directive(&self) -> bool {
        if let TokenKind::Directive = self
                { true } else { false }
    }

    /// Returns `true` if the token is unknown.
    pub fn is_unknown(&self) -> bool {
        if let TokenKind::Unknown = self
                { true } else { false }
    }

    /// Returns `true` if the token is a valid non-terminal.
    pub fn is_nonterminal(&self) -> bool {
        self.is_identifier() ||
        self.is_literal() ||
        self.is_directive()
    }
}

/// An enum which describes available keyword types.
#[derive(PartialEq, Debug, Clone)]
pub enum KeywordKind {
    Var,
    If,
    Else
}

/// An enum which describes available symbol types.
#[derive(PartialEq, Debug, Clone)]
pub enum SymbolKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    SemiColon,
    Dollar,
    Backtick,
    Hashtag,
    Address
}

/// An enum which describes available identifier types.
#[derive(PartialEq, Debug, Clone)]
pub enum OperatorKind {
    Bar,
    Caret,
    Ampersand,
    Bang,
    Equals,
    LessThan,
    GreaterThan,
    Plus,
    Minus,
    Asterisk,
    ForwardSlash,
    Percent,
    Other
}

/// An enum which describes available literal types.
#[derive(PartialEq, Debug, Clone)]
pub enum LiteralKind {
    Character,
    Integer,
    Real
}

/*
use reader::{ CharKind, StringReader };

use libcosyc_diagnostics::{ Error, ErrorKind, IssueTracker };
use libcosyc_common::source::{ Context, SourcePos };

/// A struct which produces individual `Context<TokenKind>`s.
pub struct Lexer<'a, 'b> {
    reader : StringReader<'a>,
    issues : &'b mut IssueTracker<'a>
}
impl<'a, 'b> Lexer<'a, 'b> {
    /// Creates a new lexer from this string slice.
    pub fn from(src : &'a str, issues : &'b mut IssueTracker<'a>) -> Self {
        Self {
            reader : StringReader::from(src),
            issues
        }
    }

    /// Returns the next token in the string.
    pub fn next(&mut self) -> Token<'a> {
        'search: loop {
            self.reader.clear_selection();
            let next = self.reader.next();
            let peek = self.reader.peek();
            let kind = match next {
                x if x.is_valid_whitespace() => {
                    while self.reader.peek()
                            .is_valid_whitespace() {
                        self.reader.next();
                    }
                    continue 'search;
                },
                CharKind::Minus if peek == CharKind::Minus => {
                    // line comments
                    while !self.reader.peek()
                            .is_valid_ending() {
                        self.reader.next();
                    }
                    continue 'search;
                },
                CharKind::LeftBrace if peek == CharKind::Minus => {
                    // block comments
                    self.reader.next();
                    let mut nests = 1;
                    loop {
                        let next = self.reader.next();
                        let peek = self.reader.peek();
                        match (next, peek) {
                            (_, CharKind::EoF) => {
                                self.report_error(ErrorKind::Warning, "unterminated block comment");
                                continue 'search;
                            },
                            (CharKind::LeftBrace, CharKind::Minus) => {
                                self.reader.next();
                                nests += 1
                            },
                            (CharKind::Minus, CharKind::RightBrace) => {
                                self.reader.next();
                                if nests == 1 {
                                    continue 'search;
                                } else {
                                    nests -= 1;
                                }
                            },
                            _ => ()
                        }
                    }
                },
                x if x.is_valid_digit() => {
                    while self.reader.peek()
                            .is_valid_digit() {
                        self.reader.next();
                    }
                    TokenKind::Literal(LiteralKind::Integer)
                },
                x if x.is_valid_graphic() => {
                    while self.reader.peek()
                            .is_valid_graphic() {
                        self.reader.next();
                    }
                    match self.reader.substr() {
                        "var" => TokenKind::Keyword(KeywordKind::Var),
                        "if" => TokenKind::Keyword(KeywordKind::If),
                        "else" => TokenKind::Keyword(KeywordKind::Else),
                        _ => TokenKind::Identifier
                    }
                },
                x if x.is_valid_operator() => {
                    let kind = match x {
                        CharKind::Bar => OperatorKind::Bar,
                        CharKind::Caret => OperatorKind::Caret,
                        CharKind::Ampersand => OperatorKind::Ampersand,
                        CharKind::Bang => OperatorKind::Bang,
                        CharKind::Equals => OperatorKind::Equals,
                        CharKind::LessThan => OperatorKind::LessThan,
                        CharKind::GreaterThan => OperatorKind::GreaterThan,
                        CharKind::Plus => OperatorKind::Plus,
                        CharKind::Minus => OperatorKind::Minus,
                        CharKind::Asterisk => OperatorKind::Asterisk,
                        CharKind::ForwardSlash => OperatorKind::ForwardSlash,
                        CharKind::Percent => OperatorKind::Percent,
                        _ => OperatorKind::Other
                    };
                    while self.reader.peek()
                            .is_valid_operator() {
                        self.reader.next();
                    }
                    match self.reader.substr() {
                        _ => TokenKind::Operator(kind)
                    }
                },
                CharKind::LeftParen => TokenKind::Symbol(SymbolKind::LeftParen),
                CharKind::RightParen => TokenKind::Symbol(SymbolKind::RightParen),
                CharKind::LeftBrace => TokenKind::Symbol(SymbolKind::LeftBrace),
                CharKind::RightBrace => TokenKind::Symbol(SymbolKind::RightBrace),
                CharKind::SemiColon => TokenKind::Symbol(SymbolKind::SemiColon),
                CharKind::Dollar => TokenKind::Symbol(SymbolKind::Dollar),
                CharKind::Backtick => TokenKind::Symbol(SymbolKind::Backtick),
                CharKind::Hashtag => TokenKind::Symbol(SymbolKind::Hashtag),
                CharKind::Address => TokenKind::Symbol(SymbolKind::Address),
                CharKind::EoF => TokenKind::EoF,
                _ => {
                    self.report_error(ErrorKind::Fatal, "unexpected symbol");
                    continue 'search;
                }
            };
            break 'search self.make_token(kind);
        }
    }

    /// Creates a new error of this kind and reason.
    fn report_error(&mut self, kind : ErrorKind, reason : &'static str) {
        let token = self.make_token(TokenKind::Unknown);
        self.issues.report(Error { reason, token, kind });
    }

    /// Creates a new context.
    fn make_context(&self, kind : TokenKind) -> Token<'a> {
        let line = self.reader.line();
        let context = Context {
            src : self.reader.substr(),
            line
        };
        Token { context, kind }
    }
}
*/

/*impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, Error<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        Some()
    }
}*/