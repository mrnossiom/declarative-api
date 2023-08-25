use lexer::{poor::Cursor, rich::Enricher};
use std::{fmt::Debug, fs, path::PathBuf};

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
			let enricher = Enricher::from_source(&source);

			print_all_tokens(enricher.into_iter())
		} else {
			let cursor = Cursor::from_source(&source);

			print_all_tokens(cursor.into_iter())
		}
	}
}

fn print_all_tokens<T: Debug>(iter: impl Iterator<Item = T>) {
	let _ = iter
		.inspect(|item| println!("{:?}", item))
		.collect::<Vec<_>>();
}
