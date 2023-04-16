#![feature(trait_upcasting)]

#[macro_use]
mod syntax;
mod input;

use crate::input::Input;

#[salsa::jar(db = Db)]
pub struct Jar(file);

pub trait Db: salsa::DbWithJar<Jar> + zozulya_ir::Db {}

impl<T> Db for T where T: salsa::DbWithJar<Jar> + zozulya_ir::Db {}

#[salsa::tracked]
pub fn file(db: &dyn Db, input_file: zozulya_ir::InputFile) -> zozulya_ir::SourceFile {
    let _input = Input::of(input_file.input(db));

    zozulya_ir::SourceFile::new(db, input_file)
}
