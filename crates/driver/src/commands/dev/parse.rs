use crate::commands::Act;
use dapic_parser::Parser;
use dapic_session::Session;
use std::{
	collections::hash_map::RandomState,
	error::Error,
	hash::{BuildHasher, Hasher},
	path::PathBuf,
};

#[derive(Debug, clap::Parser)]
pub(crate) struct Parse {
	file: PathBuf,
}

impl Act for Parse {
	fn act(&mut self) -> Result<(), Box<dyn Error>> {
		let mut session = Session::default();

		// TODO: create a new test case to check that we resolve correctly the char
		// positions instead of adding a dummy padding file at the beginning of the source map

		let file = session.source_map.load_file(&self.file)?;

		let root = session.enter_source_map_ctx(|session| {
			Parser::from_source(&session.parse_sess(), &file).parse_root()
		});

		match root {
			Ok(root) => {
				let name = format!(
					"/tmp/dapi-{}-{:X}.ast",
					file.source_hash,
					RandomState::new().build_hasher().finish()
				);

				std::fs::write(&name, format!("{root:#?}"))?;
				eprintln!("AST written to `{name}`");
			}
			Err(err) => session.diagnostics.emit_diagnostic(&err),
		}

		Ok(())
	}
}
