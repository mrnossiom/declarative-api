use crate::commands::Act;
use dapic_lexer::{poor::Cursor, rich::Enricher};
use dapic_session::Session;
use std::{error::Error, path::PathBuf};

#[derive(Debug, clap::Parser)]
pub(crate) struct Lex {
	file: PathBuf,

	#[clap(long)]
	rich: bool,
}

impl Act for Lex {
	fn act(&mut self) -> Result<(), Box<dyn Error>> {
		let session = Session::default();
		let file = session.parse.source_map.load_file(&self.file)?;

		if self.rich {
			Enricher::from_source(&session.parse, &file)
				.into_iter()
				.inspect(|item| println!("{item}"))
				.count();
		} else {
			Cursor::from_source(&file.source)
				.into_iter()
				// Skip whitespace that are much too verbose
				.filter(|item| !item.kind.is_whitespace())
				.inspect(|item| println!("{item}"))
				.count();
		}

		Ok(())
	}
}
