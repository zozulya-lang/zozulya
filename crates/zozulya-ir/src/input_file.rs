use crate::Symbol;

#[salsa::input]
pub struct InputFile {
    path: Symbol,

    #[return_ref]
    input: String,
}
