use parser::Parser;
use session::Session;
use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
pub(crate) struct Parse {
	file: PathBuf,
}

impl Parse {
	pub(crate) fn act(&mut self) {
		let session = Session::default();
		let file = session.parse.source_map.load_file(&self.file).unwrap();

		let mut parser = Parser::from_source(&session.parse, &file);

		match parser.parse_root() {
			Ok(root) => println!("{root:?}"),
			Err(err) => session.parse.diagnostic.emit_diagnostic(err),
		}
	}
}
