//! Declarative API `AST` expansion
//!
//! Multiple passes are done on the `AST`:
//! - [`ScopeExpander`] visits `scope` items and loads external ones by loading
//!   accoding files.
//! - [`NodeExpander`] renumbers every `AST` item node so each get a unique one.

use crate::scope::ScopeData;
pub use crate::{node::NodeExpander, scope::ScopeExpander};
use dapic_ast::{types::Ast, visit_mut::MutVisitor};
use dapic_session::{symbols::kw, Ident, Session, Span};

mod errors;
mod node;
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

	// First load all external scopes
	ScopeExpander { session, scope }.visit_root(api);
	// Renumber every node to replace dummy ones
	NodeExpander::default().visit_root(api);
}
