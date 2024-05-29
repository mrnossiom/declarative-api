use crate::commands::Act;
use dapic_expand::expand_ast;
use dapic_parser::Parser;
use dapic_session::Session;
use std::{
	collections::hash_map::RandomState,
	error::Error,
	hash::{BuildHasher, Hasher},
	path::PathBuf,
};

#[derive(Debug, clap::Parser)]
pub(crate) struct Expand {
	file: PathBuf,
}

impl Act for Expand {
	fn act(&mut self) -> Result<(), Box<dyn Error>> {
		let mut session = Session::default();
		let file = session.source_map.load_file(&self.file)?;
		let root = session.enter_source_map_ctx(|session| {
			Parser::from_source(&session.parse_sess(), &file).parse_root()
		});

		let filename = format!(
			"/tmp/dapi-{}-{:X}",
			file.source_hash,
			RandomState::new().build_hasher().finish()
		);

		let mut root = match root {
			Ok(root) => {
				let name = format!("{filename}.ast");
				std::fs::write(&name, format!("{root:#?}"))?;
				eprintln!("AST written to `{name}`");

				root
			}
			Err(err) => session.diagnostics.emit_fatal_diagnostic(&err),
		};

		expand_ast(&session, &mut root);

		let name = format!("{filename}.east");
		std::fs::write(&name, format!("{root:#?}"))?;
		eprintln!("AST expanded written to `{name}`");

		Ok(())
	}
}
