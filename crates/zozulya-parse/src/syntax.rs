#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum Syntax {
    INT,
    COLON,
    EQ,
    IDENT,
    ASSIGN,
    WHITESPACE,
    UNKNOWN,
    EOF,
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
}
