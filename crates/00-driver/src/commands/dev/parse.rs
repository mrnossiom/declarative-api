use crate::commands::Act;
use parser::Parser;
use session::{add_source_map_context, Session};
use std::{error::Error, path::PathBuf};

#[derive(Debug, clap::Parser)]
pub(crate) struct Parse {
	file: PathBuf,
}

impl Act for Parse {
	fn act(&mut self) -> Result<(), Box<dyn Error>> {
		let session = Session::default();

		// TODO: remove this sourcemap shift hack ;)
		let _anon = session
			.parse
			.source_map
			.load_anon((0..100).map(|int| int.to_string()).collect());

		let file = session.parse.source_map.load_file(&self.file)?;

		add_source_map_context(session.parse.source_map.clone(), || {
			let mut parser = Parser::from_source(&session.parse, &file);

			match parser.parse_root() {
				Ok(root) => println!("{root:?}"),
				Err(err) => session.parse.diagnostic.emit_diagnostic(&err),
			}
		});

		Ok(())
	}
}
