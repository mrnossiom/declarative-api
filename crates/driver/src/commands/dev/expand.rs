use crate::commands::Act;
use expand::expand_ast;
use parser::Parser;
use session::{add_source_map_context, Session};
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
		let session = Session::default();
		let file = session.parse.source_map.load_file(&self.file)?;
		let root = add_source_map_context(session.parse.source_map.clone(), || {
			Parser::from_source(&session.parse, &file).parse_root()
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
			Err(err) => {
				session.parse.diagnostic.emit_diagnostic(&err);
				panic!()
			}
		};

		expand_ast(&mut root);

		let name = format!("{filename}.east");
		std::fs::write(&name, format!("{root:#?}"))?;
		eprintln!("AST expanded written to `{name}`");

		Ok(())
	}
}
