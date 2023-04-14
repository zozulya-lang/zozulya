#![feature(trait_upcasting)]

mod lexer;
mod syntax;

#[salsa::jar(db = Db)]
pub struct Jar(file);

pub trait Db: salsa::DbWithJar<Jar> + zozulya_ir::Db {}

impl<T> Db for T where T: salsa::DbWithJar<Jar> + zozulya_ir::Db {}

#[salsa::tracked]
pub fn file(db: &dyn Db, input_file: zozulya_ir::InputFile) -> zozulya_ir::SourceFile {
    let tokens = lexer::lex(db, input_file);

    println!("{tokens:?}");

    zozulya_ir::SourceFile::new(db, input_file, Vec::new())
}
