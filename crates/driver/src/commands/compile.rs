use crate::commands::Act;
use expand::expand_ast;
use parser::Parser;
use session::{add_source_map_context, Session};
use std::{error::Error, path::PathBuf};

#[derive(Debug, clap::Parser)]
pub(crate) struct Compile {
	file: PathBuf,
}

impl Act for Compile {
	fn act(&mut self) -> Result<(), Box<dyn Error>> {
		let mut session = Session::default();

		let file = session
			.time("load_file")
			.run(|| session.parse.source_map.load_file(&self.file))?;

		add_source_map_context(session.parse.source_map.clone(), || {
			let mut api = session
				.time("parse")
				.run(|| Parser::from_source(&session.parse, &file).parse_root())
				.map_err(|err| session.parse.diagnostic.emit_diagnostic(&err))
				.unwrap();

			session
				.time("expand")
				.run(|| expand_ast(&session, &mut api));
		});

		Ok(())
	}
}
