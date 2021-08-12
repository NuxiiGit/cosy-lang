use libcosyc_diagnostic::source::Span;
use std::{ str::CharIndices, mem };
use crate::symbol::SymbolKind;

/// Iterates over characters of a string and produces useful substrings and tagged data.
pub struct SymbolReader<'a> {
    src : &'a str,
    chars : CharIndices<'a>,
    current : SymbolKind,
    span : Span,
    only_dashes : bool
}

impl<'a> SymbolReader<'a> {
    /// Returns the current span.
    pub fn span(&self) -> &Span {
        &self.span
    }

    /// Clears the current span.
    pub fn reset_span(&mut self) {
        self.span.begin = self.span.end;
        self.only_dashes = true;
    }

    /// Returns the current substring.
    pub fn substring(&self) -> &'a str {
        self.span.render(self.src)
    }

    /// Peeks at the next `SymbolKind`.
    pub fn peek(&self) -> &SymbolKind {
        &self.current
    }

    /// Advances the reader and returns the next `SymbolKind`.
    pub fn advance(&mut self) -> SymbolKind {
        let future = if let Some((i, c)) = self.chars.next() {
            self.span.end = i;
            SymbolKind::identify(c)
        } else {
            self.span.end = self.src.len();
            SymbolKind::EoF
        };
        if self.only_dashes && !matches!(self.current, SymbolKind::Minus) {
            self.only_dashes = false;
        }
        mem::replace(&mut self.current, future)
    }

    /// Advances the reader whilst some predicate holds.
    /// Always halts if the `EoF` character is reached.
    pub fn advance_while(&mut self, p : fn(&SymbolKind) -> bool) {
        loop {
            match &self.current {
                SymbolKind::EoF => break,
                x if p(x) => { self.advance(); },
                _ => break
            }
        }
    }

    /// Returns whether the current lexeme is a comment.
    pub fn holds_comment_lexeme(&self) -> bool {
        self.only_dashes && self.span.length() > 1
    }
}

impl<'a> From<&'a str> for SymbolReader<'a> {
    fn from(src : &'a str) -> Self {
        let mut chars = src.char_indices();
        let current = chars
                .next()
                .map(|(_, snd)| SymbolKind::identify(snd))
                .unwrap_or(SymbolKind::EoF);
        let span = Span::default();
        let only_dashes = true;
        Self { src, chars, current, span, only_dashes }
    }
}
