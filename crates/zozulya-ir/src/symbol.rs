#[salsa::interned]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Symbol {
    #[return_ref]
    pub text: Box<str>,
}

impl Symbol {
    pub fn intern(db: &dyn crate::Db, text: impl Into<Box<str>>) -> Self {
        Self::new(db, text.into())
    }
}
