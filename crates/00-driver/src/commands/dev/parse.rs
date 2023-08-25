use parser::Parser;
use std::{fs, path::PathBuf};

#[derive(Debug, clap::Parser)]
pub(crate) struct Parse {
	file: PathBuf,
}

impl Parse {
	pub(crate) fn act(&mut self) {
		let source = fs::read_to_string(&self.file).unwrap();

		let mut parser = Parser::from_source(&source);

		println!("{:?}", parser.parse_api().unwrap())
	}
}
