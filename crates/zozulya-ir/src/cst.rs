#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Zozulya;

impl rowan::Language for Zozulya {
    type Kind = ZozulyaKind;

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
pub enum ZozulyaKind {
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

impl From<u16> for ZozulyaKind {
    fn from(value: u16) -> Self {
        assert!(value <= (Self::EOF as u16));
        unsafe { std::mem::transmute(value) }
    }
}

impl From<ZozulyaKind> for u16 {
    fn from(syntax: ZozulyaKind) -> Self {
        syntax as Self
    }
}

impl From<ZozulyaKind> for rowan::SyntaxKind {
    fn from(value: ZozulyaKind) -> Self {
        Self(value.into())
    }
}

impl ZozulyaKind {
    pub fn is_trivia(self) -> bool {
        matches!(self, Self::WHITESPACE)
    }
}
