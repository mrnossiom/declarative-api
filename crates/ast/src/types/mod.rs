use dapic_session::{new_index_ty, Span};
use thin_vec::ThinVec;

mod attr;
mod expr;
mod item;

pub use crate::ptr::P;
pub use attr::*;
pub use expr::*;
pub use item::*;

#[derive(Debug, Clone)]
pub struct Ast {
	pub attrs: AttrVec,
	pub items: ThinVec<P<Item>>,

	pub id: NodeId,
	pub span: Span,
}

new_index_ty! {
	/// Identifies an AST node.
	///
	/// This identifies top-level definitions, expressions, and everything in between.
	pub struct NodeId;
}

impl NodeId {
	/// The [`NodeId`] used to represent the root of the crate.
	pub const ROOT: Self = Self(0);

	/// We initially give all AST nodes this dummy [`NodeId`] value. Those
	/// are renumbered during expansion.
	pub const DUMMY: Self = Self(usize::MAX);
}
