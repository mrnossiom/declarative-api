use crate::P;

mod attr;
mod expr;
mod item;
mod node;
pub use attr::*;
pub use expr::*;
pub use item::*;
pub use node::*;
use session::{Ident, Span};
use thin_vec::ThinVec;

#[derive(Debug, Clone)]
pub struct Api {
	pub attrs: AttrVec,
	pub items: ThinVec<P<Item>>,

	pub id: NodeId,
	pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ty {
	pub kind: TyKind,
	pub id: NodeId,
	pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TyKind {
	/// The base type, either
	/// - a path: `scope.Type`
	/// - or a local type: `Type`
	Path(Path),

	/// An array of types: `[Type]`
	Array(P<Ty>),

	/// A tuple of types: `(Ty1, Ty2, Ty3)`
	/// Can also define the unit type: `()`
	Tuple(ThinVec<P<Ty>>),

	Paren(P<Ty>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path {
	pub segments: ThinVec<PathSegment>,
	pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathSegment {
	pub ident: Ident,
	pub id: NodeId,
}
