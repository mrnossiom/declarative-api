use crate::P;

mod attr;
mod expr;
mod item;
mod node;
pub use attr::*;
pub use expr::*;
pub use item::*;
pub use node::*;
use session::Span;
use thin_vec::ThinVec;

#[derive(Debug, Clone)]
pub struct Api {
	pub attrs: AttrVec,
	pub items: ThinVec<P<Item>>,

	pub id: NodeId,
	pub span: Span,
}

// TODO: nope, not a String
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type(pub String);
