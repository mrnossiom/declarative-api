use crate::errors::scope::{
	CyclicImport, LoadingError, MultipleCandidates, NoCandidate, ParsingError,
};
use dapic_ast::{
	types::{AttrVec, Item, ItemKind, Root, ScopeKind, P},
	visit_mut::{noop, MutVisitor},
};
use dapic_parser::Parser;
use dapic_session::{symbols::kw, Diagnostic, Ident, Session, Span};
use std::{
	fmt::Write,
	path::{self, PathBuf},
};
use thin_vec::ThinVec;

#[derive(Debug)]
pub struct ScopeExpander<'a> {
	pub(crate) session: &'a Session,
	pub(crate) scope: Option<ScopeData>,
}

impl<'a> ScopeExpander<'a> {
	/// Resolves entrypoint source file and uses it to resolve external scopes.
	///
	/// # Panics
	/// If entrypoint filepath doesn't have a parent
	pub fn expand(session: &'a Session, ast: &mut Root) {
		let entrypoint = session.source_map.lookup_source_file(ast.span.high());

		let scope = entrypoint.name.clone().into_real().map(|file| {
			let dir_path = file.parent().expect("path should not be empty").to_owned();

			ScopeData {
				// TODO: change to real root api name
				mod_path: vec![Ident::new(kw::PathRoot, Span::DUMMY)],
				file_path_stack: vec![file],
				dir_path,
			}
		});

		Self { session, scope }.visit_root(ast);
	}
}

#[derive(Debug)]
pub struct ScopeData {
	pub mod_path: Vec<Ident>,
	pub file_path_stack: Vec<PathBuf>,
	pub dir_path: PathBuf,
}

impl ScopeData {
	pub(crate) fn with_dir_path(&self, dir_path: PathBuf) -> Self {
		Self {
			mod_path: self.mod_path.clone(),
			file_path_stack: self.file_path_stack.clone(),
			dir_path,
		}
	}
}

type ExtScopeReturn = (ThinVec<P<Item>>, Span, Option<PathBuf>, PathBuf);

impl<'a> ScopeExpander<'a> {
	fn load_external_scope(
		&self,
		ident: Ident,
		scope: Span,
		attrs: &mut AttrVec,
		scope_data: &ScopeData,
	) -> Result<ExtScopeReturn, Diagnostic> {
		// IDEA: behave differently when in a `<ident>.dapi` and not a `scope.dapi` file? adding a prefix like `{{parent}}/<ident>/...`

		// `<ident>.dapi`
		let sibling_path = format!("{}.dapi", ident.symbol);
		let sibling_path = scope_data.dir_path.join(sibling_path);
		// `<ident>/scope.dapi`
		let child_path = format!("{}{}scope.dapi", ident.symbol, path::MAIN_SEPARATOR);
		let child_path = scope_data.dir_path.join(child_path);

		let file_path = match (sibling_path.exists(), child_path.exists()) {
			(true, false) => sibling_path,
			(false, true) => child_path,
			(true, true) => {
				// Both files exist, so we can't load the scope
				return Err(MultipleCandidates {
					import_name: ident,
					import: scope,
					child_candidate: child_path.to_string_lossy().into_owned(),
					sibling_candidate: sibling_path.to_string_lossy().into_owned(),
				}
				.into());
			}
			(false, false) => {
				// Neither file exists, so we can't load the scope
				return Err(NoCandidate {
					import_name: ident,
					import: scope,
					child_candidate: child_path.to_string_lossy().into_owned(),
					sibling_candidate: sibling_path.to_string_lossy().into_owned(),
				}
				.into());
			}
		};

		// Ensure file paths are acyclic.
		if let Some(pos) = scope_data
			.file_path_stack
			.iter()
			.position(|p| p == &file_path)
		{
			return Err(CyclicImport {
				import_name: ident,
				import: scope,
				import_stack: scope_data.file_path_stack[pos..].to_vec().iter().fold(
					String::new(),
					|mut s, p| {
						write!(s, "{}", p.display()).unwrap();
						s
					},
				),
			}
			.into());
		}

		let source = self
			.session
			.source_map
			.load_file(&file_path)
			.map_err(|io| {
				LoadingError {
					import_name: ident,
					import: scope,
					io,
				}
				.into()
			})?;

		let psess = self.session.parse_sess();
		let mut parser = Parser::from_source(&psess, &source);

		let lo = parser.token.span;
		let items = match parser.parse_scope_content(Some(attrs)) {
			Ok(items) => items,
			Err(err) => {
				self.session.diagnostics.emit_diagnostic(&err);

				return Err(ParsingError {
					import_name: ident,
					import: scope,
					parsing_err: err,
				}
				.into());
			}
		};
		let span = lo.to(parser.prev_token.span);

		let dir = scope_data.dir_path.join(ident.symbol.as_str());

		Ok((items, span, Some(file_path), dir))
	}
}

impl<'a> MutVisitor for ScopeExpander<'a> {
	fn visit_item(&mut self, item: &mut P<Item>) {
		let ItemKind::Scope(kind) = &mut item.kind else {
			noop::visit_item(self, item);
			return;
		};

		let Some(scope_data) = self.scope.take() else {
			panic!("it's impossible to expand external scopes in anonymous files")
		};

		let (file_path, dir_path) = match kind {
			ScopeKind::Loaded { .. } => {
				(None, scope_data.dir_path.join(item.ident.symbol.as_str()))
			}
			ScopeKind::Unloaded => {
				let (items, span, file_path, dir_path) = self
					.load_external_scope(item.ident, item.span, &mut item.attrs, &scope_data)
					.map_err(|diag| self.session.diagnostics.emit_diagnostic(&diag))
					.unwrap_or_default();

				// Mutate the AST to add the newly loaded scope
				// In case an error was emitted, we still want to return a loaded dummy scope
				item.kind = ItemKind::Scope(ScopeKind::Loaded {
					items,
					inline: false,
					span,
				});

				(file_path, dir_path)
			}
		};

		let mut mod_data = scope_data.with_dir_path(dir_path);
		mod_data.mod_path.push(item.ident);
		if let Some(path) = file_path {
			mod_data.file_path_stack.push(path);
		}

		self.scope.replace(mod_data);
		noop::visit_item(self, item);

		// Step down, put original context
		self.scope.replace(scope_data);
	}
}
