#![warn(
	// clippy::missing_docs_in_private_items,
	clippy::unwrap_used,
	clippy::nursery,
	clippy::pedantic,
	clippy::todo,
	clippy::dbg_macro,
	rustdoc::all,
)]
#![allow(
	clippy::redundant_pub_crate,
	clippy::enum_glob_use,
	clippy::module_name_repetitions,
	clippy::missing_errors_doc
)]

use ast::visitor::MutVisitor;
use scope::{ModuleData, ScopeExpander};
use session::{symbols::kw, Ident, Session, Span};

mod errors;
mod scope;

pub fn expand_ast(session: &Session, api: &mut ast::types::Api) {
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
