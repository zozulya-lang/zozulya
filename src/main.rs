#![feature(exact_size_is_empty)]

use miette::{bail, Context, IntoDiagnostic, Result};
use zozulya_db::Database;
use zozulya_ir::{InputFile, Symbol};

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);

    match args.next() {
        Some(path) if args.is_empty() => {
            let input = std::fs::read_to_string(&path)
                .into_diagnostic()
                .with_context(|| format!("reading `{path}`"))?;

            let db = Database::default();

            let path = Symbol::intern(&db, path);
            let input_file = InputFile::new(&db, path, input);

            dbg!(input_file);
            Ok(())
        }
        Some(_) => bail!("you must specify exactly one input file"),
        None => bail!("you must specify an input file"),
    }
}
