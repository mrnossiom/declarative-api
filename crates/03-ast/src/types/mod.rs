use crate::P;
use lexer::{span::Span, symbols::Symbol};

mod attr;
mod item;
mod node;
pub use attr::*;
pub use item::*;
pub use node::*;

#[derive(Debug)]
pub struct Api {
	pub attrs: AttrVec,
	pub meta: Metadata,
	pub items: Vec<P<Item>>,

	pub id: NodeId,
	pub span: Span,
}

#[derive(Debug)]
pub struct Metadata {
	// pub name: String,
	// pub description: String,
	// pub licence: String,
	// pub urls: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Type(pub(crate) String);

#[derive(Debug, Clone)]
pub struct Ident {
	pub symbol: Symbol,
	pub span: Span,
}
