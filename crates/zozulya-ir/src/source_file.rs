#[salsa::tracked]
pub struct SourceFile {
    #[id]
    pub input_file: crate::InputFile,
}
