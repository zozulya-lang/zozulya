#![feature(trait_upcasting)]

mod input;
mod parser;
mod sink;

use crate::input::Input;
use crate::parser::Parser;

#[salsa::jar(db = Db)]
pub struct Jar(file);

pub trait Db: salsa::DbWithJar<Jar> + zozulya_ir::Db {}

impl<T> Db for T where T: salsa::DbWithJar<Jar> + zozulya_ir::Db {}

#[salsa::tracked]
pub fn file(db: &dyn Db, input_file: zozulya_ir::InputFile) -> zozulya_ir::SourceFile {
    let input = Input::of(input_file.input(db));
    let parser = Parser::new(input);
    let green = parser.parse();
    zozulya_ir::SourceFile::new(db, input_file, green)
}
