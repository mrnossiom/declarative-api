//! Declarative API `AST` expansion
//!
//! Multiple passes are done on the `AST`:
//! - [`ScopeExpander`] visits `scope` items and loads external ones by loading
//!   accoding files.
//! - [`NodeExpander`] renumbers every `AST` item node so each get a unique one.

pub use crate::{node::NodeExpander, scope::ScopeExpander};
use dapic_ast::{types::Root, visit_mut::MutVisitor};
use dapic_session::Session;

mod errors;
mod node;
mod scope;

pub fn expand_ast(session: &Session, ast: &mut Root) {
	// First load all external scopes
	ScopeExpander::expand(session, ast);
	// Renumber every node to replace dummy ones
	NodeExpander::default().visit_root(ast);
}
