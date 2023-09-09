use crate::commands::Act;
use parser::Parser;
use session::Session;
use std::{error::Error, path::PathBuf};

#[derive(Debug, clap::Parser)]
pub(crate) struct Parse {
	file: PathBuf,
}

impl Act for Parse {
	fn act(&mut self) -> Result<(), Box<dyn Error>> {
		let session = Session::default();
		let _anon = session.parse.source_map.load_anon("22".into());
		let file = session.parse.source_map.load_file(&self.file)?;

		let mut parser = Parser::from_source(&session.parse, &file);

		match parser.parse_root() {
			Ok(root) => println!("{root:?}"),
			Err(err) => session.parse.diagnostic.emit_diagnostic(err),
		}

		Ok(())
	}
}
