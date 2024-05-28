use crate::commands::Act;
use dapic_expand::expand_ast;
use dapic_parser::Parser;
use dapic_session::{add_source_map_context, Session};
use std::{error::Error, path::PathBuf};

#[derive(Debug, clap::Parser)]
pub(crate) struct Compile {
	file: PathBuf,
}

impl Act for Compile {
	fn act(&mut self) -> Result<(), Box<dyn Error>> {
		let mut session = Session::default();

		let file = session
			.time("srcmap_load_file")
			.run(|| session.parse.source_map.load_file(&self.file))?;

		add_source_map_context(session.parse.source_map.clone(), || {
			let Ok(mut ast) = session
				.time("ast_parse")
				.run(|| Parser::from_source(&session.parse, &file).parse_root())
				.map_err(|err| session.parse.diagnostic.emit_fatal_diagnostic(&err))
			else {
				unreachable!("above pattern is Result<Ast, !>, which means only valid pat is Ok(_)")
			};

			session
				.time("ast_expansion")
				.run(|| expand_ast(&session, &mut ast));

			let hir = session
				.time("hir_creation")
				.run(|| dapic_hir::compile_hir(&ast));

			session.time("ast_drop").run(|| drop(ast));
		});

		Ok(())
	}
}
