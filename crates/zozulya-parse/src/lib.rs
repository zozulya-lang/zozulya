#![feature(trait_upcasting)]

#[macro_use]
mod syntax;
mod input;
mod parser;
mod sink;

use rowan::SyntaxNode;

use crate::input::Input;
use crate::parser::Parser;
use crate::syntax::Zozulya;

#[salsa::jar(db = Db)]
pub struct Jar(file);

pub trait Db: salsa::DbWithJar<Jar> + zozulya_ir::Db {}

impl<T> Db for T where T: salsa::DbWithJar<Jar> + zozulya_ir::Db {}

#[salsa::tracked]
pub fn file(db: &dyn Db, input_file: zozulya_ir::InputFile) -> zozulya_ir::SourceFile {
    let input = Input::of(input_file.input(db));
    let parser = Parser::new(input);
    let green = parser.parse();

    dbg!(SyntaxNode::<Zozulya>::new_root(green));

    zozulya_ir::SourceFile::new(db, input_file)
}
