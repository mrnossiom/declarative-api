use lexer::{poor::Cursor, rich::Enricher};
use session::Session;
use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
pub(crate) struct Lex {
	file: PathBuf,

	#[clap(long)]
	rich: bool,
}

impl Lex {
	pub(crate) fn act(&mut self) {
		let session = Session::default();
		let file = session.parse.source_map.load_file(&self.file).unwrap();

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
	}
}
