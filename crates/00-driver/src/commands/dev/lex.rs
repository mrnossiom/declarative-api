use lexer::{poor::Cursor, rich::Enricher};
use std::{fs, path::PathBuf};

#[derive(Debug, clap::Parser)]
pub(crate) struct Lex {
	file: PathBuf,

	#[clap(long)]
	rich: bool,
}

impl Lex {
	pub(crate) fn act(&mut self) {
		let source = fs::read_to_string(&self.file).unwrap();

		if self.rich {
			Enricher::from_source(&source)
				.into_iter()
				.inspect(|item| println!("{}", item))
				.count();
		} else {
			Cursor::from_source(&source)
				.into_iter()
				// Skip whitespace that are much too verbose
				.filter(|item| !item.kind.is_whitespace())
				.inspect(|item| println!("{}", item))
				.count();
		}
	}
}
