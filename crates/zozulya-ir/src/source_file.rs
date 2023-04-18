use rowan::GreenNode;

#[salsa::tracked]
pub struct SourceFile {
    #[id]
    pub input_file: crate::InputFile,

    pub cst: GreenNode,
}
