use libcosyc_diagnostic::source::Span;

/// Represents a kind of term.
#[derive(Debug)]
pub enum TermKind {
    Variable,
    Integral {
        radix : u8
    },
    TypeAnno {
        value : Box<Term>,
        datatype : Box<Term>
    },
    BinaryOp {
        op : Span,
        left : Box<Term>,
        right : Box<Term>
    },
    UnaryOp {
        op : Span,
        value : Box<Term>
    }
}

/// Represents AST terms.
#[derive(Debug)]
pub struct Term {
    pub span : Span,
    pub kind : TermKind
}
