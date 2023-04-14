mod input_file;
mod symbol;

pub use input_file::InputFile;
pub use symbol::Symbol;

#[salsa::jar(db = Db)]
pub struct Jar(InputFile, Symbol);

pub trait Db: salsa::DbWithJar<Jar> {
    fn as_dyn_ir_db(&self) -> &dyn Db;
}

impl<T: salsa::DbWithJar<Jar>> Db for T {
    fn as_dyn_ir_db(&self) -> &dyn Db {
        self
    }
}
