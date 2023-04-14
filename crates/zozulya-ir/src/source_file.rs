#[salsa::tracked]
pub struct SourceFile {
    #[id]
    pub input_file: crate::InputFile,

    #[return_ref]
    stmts: Vec<()>,
}
