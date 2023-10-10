use crate::errors::ExtScopeLoadingError;
use ast::{
	types::{ItemKind, ScopeKind},
	visitor::{noop, MutVisitor},
};
use parser::Parser;
use session::{Ident, Session};
use std::{
	mem,
	path::{self, PathBuf},
};
use thin_vec::thin_vec;

#[derive(Debug)]
pub struct ScopeExpander<'a> {
	session: &'a Session,
	mod_data: ModuleData,
}

#[derive(Debug)]
pub struct ModuleData {
	pub mod_path: Vec<Ident>,
	pub file_path_stack: Vec<PathBuf>,
	pub dir_path: PathBuf,
}

impl ModuleData {
	pub(crate) fn with_dir_path(&self, dir_path: PathBuf) -> Self {
		Self {
			mod_path: self.mod_path.clone(),
			file_path_stack: self.file_path_stack.clone(),
			dir_path,
		}
	}
}

impl<'a> ScopeExpander<'a> {
	pub const fn new(session: &'a Session, mod_data: ModuleData) -> Self {
		ScopeExpander { session, mod_data }
	}
}

impl<'a> MutVisitor for ScopeExpander<'a> {
	fn visit_item(&mut self, item: &mut ast::types::P<ast::types::Item>) {
		let ItemKind::Scope(kind) = &mut item.kind else {
			noop::visit_item(self, item);
			return;
		};

		// TODO: this could be reworked by taking these out as functions that return an error
		// this would be even prettier if we would support enums in diagnostics
		// like a big chungus mod error enum with variants like `FileNotFound`, `FileNotDapiExt`, `ManyCandidates`,`IoError` or `ParsingError`
		let (file_path, dir_path) = match kind {
			ScopeKind::Loaded { .. } => {
				let dir = self.mod_data.dir_path.join(item.ident.symbol.as_str());

				(None, dir)
			}

			ScopeKind::Unloaded => {
				// IDEA: behave differently when in a `<ident>.dapi`` and not a `scope.dapi` file? adding a prefix like `{{parent}}/<ident>/...`

				// `<ident>.dapi`
				let sibling_path = format!("{}.dapi", item.ident.symbol);
				let sibling_path = self.mod_data.dir_path.join(sibling_path);
				// `<ident>/scope.dapi`
				let child_path = format!("{}{}scope.dapi", item.ident.symbol, path::MAIN_SEPARATOR);
				let child_path = self.mod_data.dir_path.join(child_path);

				let file_path = match (sibling_path.exists(), child_path.exists()) {
					(true, false) => sibling_path,
					(false, true) => child_path,
					(true, true) => {
						// Both files exist, so we can't load the scope
						todo!("emit error")
					}
					(false, false) => {
						// Neither file exists, so we can't load the scope
						todo!("emit error")
					}
				};

				let source = match self.session.parse.source_map.load_file(&file_path) {
					Ok(source) => source,
					Err(io_error) => {
						self.session.parse.diagnostic.emit(ExtScopeLoadingError {
							import: item.span,
							name: item.ident,
							io_error,
						});
						return;
					}
				};

				let mut parser = Parser::from_source(&self.session.parse, &source);

				let lo = parser.token.span;
				let items = match parser.parse_scope_content(Some(&mut item.attrs)) {
					Ok(items) => items,
					Err(err) => {
						self.session.parse.diagnostic.emit_diagnostic(&err);

						// Return empty items instead of leaving the scope in an unloaded state
						thin_vec![]
					}
				};
				let span = lo.to(parser.prev_token.span);

				// Mutate the AST to add the newly loaded scope
				item.kind = ItemKind::Scope(ScopeKind::Loaded {
					items,
					inline: false,
					span,
				});

				let dir = self.mod_data.dir_path.join(item.ident.symbol.as_str());

				(Some(file_path), dir)
			}
		};

		let mut mod_data = self.mod_data.with_dir_path(dir_path);
		mod_data.mod_path.push(item.ident);
		if let Some(path) = file_path {
			mod_data.file_path_stack.push(path);
		}
		let original_mod_data = mem::replace(&mut self.mod_data, mod_data);

		noop::visit_item(self, item);

		self.mod_data = original_mod_data;
	}
}
