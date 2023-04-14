use crate::Symbol;

#[salsa::input]
pub struct InputFile {
    pub path: Symbol,

    #[return_ref]
    pub input: String,
}
