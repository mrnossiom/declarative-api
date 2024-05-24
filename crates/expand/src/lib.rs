use dapic_ast::visitor::MutVisitor;
use dapic_session::{symbols::kw, Ident, Session, Span};
use scope::{ModuleData, ScopeExpander};

mod errors;
mod scope;

pub fn expand_ast(session: &Session, api: &mut dapic_ast::types::Api) {
	let file_path = session
		.parse
		.source_map
		.lookup_source_file(api.span.high())
		.name
		.clone()
		.into_real()
		.unwrap();

	let dir_path = file_path
		.parent()
		.expect("path should not be empty")
		.to_owned();

	let mod_data = ModuleData {
		// TODO: change to real root api name
		mod_path: vec![Ident::new(kw::PathRoot, Span::DUMMY)],
		file_path_stack: vec![file_path],
		dir_path,
	};

	ScopeExpander::new(session, mod_data).visit_root(api);
}
