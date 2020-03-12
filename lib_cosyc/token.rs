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