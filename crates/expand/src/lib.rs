use crate::scope::{ScopeData, ScopeExpander};
use dapic_ast::{types::Ast, visitor::MutVisitor};
use dapic_session::{symbols::kw, Ident, Session, Span};

mod errors;
mod scope;

pub fn expand_ast(session: &Session, api: &mut Ast) {
	// TODO: handle anon files
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

	let scope = ScopeData {
		// TODO: change to real root api name
		mod_path: vec![Ident::new(kw::PathRoot, Span::DUMMY)],
		file_path_stack: vec![file_path],
		dir_path,
	};

	ScopeExpander { session, scope }.visit_root(api);
}
