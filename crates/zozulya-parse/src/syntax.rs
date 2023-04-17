#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Zozulya;

impl rowan::Language for Zozulya {
    type Kind = Syntax;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        raw.0.into()
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.into())
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[allow(clippy::upper_case_acronyms)]
#[repr(u16)]
pub enum Syntax {
    LITERAL,
    INT,
    LOCAL,
    COLON,
    EQ,
    IDENT,
    ASSIGN,
    WHITESPACE,
    UNKNOWN,
    TOMBSTONE,
    EOF,
}

impl From<u16> for Syntax {
    fn from(value: u16) -> Self {
        assert!(value <= (Self::EOF as u16));
        unsafe { std::mem::transmute(value) }
    }
}

impl From<Syntax> for u16 {
    fn from(syntax: Syntax) -> Self {
        syntax as Self
    }
}

impl Syntax {
    pub fn is_trivia(self) -> bool {
        matches!(self, Self::WHITESPACE)
    }
}

#[rustfmt::skip]
#[macro_export]
macro_rules! T {
    (:) => { $crate::syntax::Syntax::COLON };
    (:=) => { $crate::syntax::Syntax::ASSIGN };
    (=) => { $crate::syntax::Syntax::EQ };
    (@ident) => { $crate::syntax::Syntax::IDENT };
    (@int) => { $crate::syntax::Syntax::INT };
    (@whitespace) => { $crate::syntax::Syntax::WHITESPACE };
    (@unknown) => { $crate::syntax::Syntax::UNKNOWN };
    (@eof) => { $crate::syntax::Syntax::EOF };
    (@local) => { $crate::syntax::Syntax::LOCAL };
    (@tombstone) => { $crate::syntax::Syntax::TOMBSTONE };
    (@literal) => { $crate::syntax::Syntax::LITERAL };
}
