use dapic_session::Span;
use thin_vec::ThinVec;

mod attr;
mod expr;
mod item;
mod node;
pub use crate::ptr::P;
pub use attr::*;
pub use expr::*;
pub use item::*;
pub use node::*;

#[derive(Debug, Clone)]
pub struct Api {
	pub attrs: AttrVec,
	pub items: ThinVec<P<Item>>,

	pub id: NodeId,
	pub span: Span,
}
