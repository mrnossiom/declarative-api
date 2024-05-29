use crate::commands::Act;
use dapic_expand::expand_ast;
use dapic_parser::Parser;
use dapic_session::Session;
use std::{error::Error, path::PathBuf};

#[derive(Debug, clap::Parser)]
pub(crate) struct Compile {
	file: PathBuf,
}

impl Act for Compile {
	fn act(&mut self) -> Result<(), Box<dyn Error>> {
		let mut session = Session::default();

		session.enter_source_map_ctx(|session| {
			// Load entrypoint file
			let file = session.source_map.load_file(&self.file)?;

			// Parse initial file AST
			let Ok(mut ast) = session
				.time("ast_parse")
				.run(|| Parser::from_source(&session.parse_sess(), &file).parse_root())
				.map_err(|err| session.diagnostics.emit_fatal_diagnostic(&err))
			else {
				unreachable!("above pattern is Result<Ast, !>, which means only valid pat is Ok(_)")
			};

			session.diagnostics.check_degraded_and_exit();

			// Expand AST (load external scopes, assign NodeIds)
			session
				.time("ast_expansion")
				.run(|| expand_ast(session, &mut ast));

			session.diagnostics.check_degraded_and_exit();

			// Build HIR
			let _hir = session
				.time("hir_creation")
				.run(|| dapic_hir::compile_hir(&ast));

			session.diagnostics.check_degraded_and_exit();

			// AST is not needed anymore
			session.time("ast_drop").run(|| drop(ast));

			Ok::<_, Box<dyn Error>>(())
		})?;

		session.diagnostics.print_final_stats();

		Ok(())
	}
}
