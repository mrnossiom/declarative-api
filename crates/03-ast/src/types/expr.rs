use super::{AttrVec, Ident, NodeId};
use crate::P;
use lexer::{rich::LiteralKind, span::Span, symbols::Symbol};
use thin_vec::ThinVec;

/// An expression.
#[derive(Debug, Clone)]
pub struct Expr {
	pub attrs: AttrVec,
	pub kind: ExprKind,
	// pub tokens: Option<LazyAttrTokenStream>,
	pub id: NodeId,
	pub span: Span,
}

#[derive(Debug, Clone)]
pub enum ExprKind {
	/// An array (`[a, b, c, d]`)
	Array(ThinVec<P<Expr>>),
	/// A literal (e.g., `1`, `"foo"`).
	Literal(LiteralKind, Symbol),
}

/// A single field in a struct expression, e.g. `x: value` and `y` in `Foo { x: value, y }`.
#[derive(Debug, Clone)]
pub struct ExprField {
	pub attrs: AttrVec,
	pub ident: Ident,
	pub expr: P<Expr>,

	pub id: NodeId,
	pub span: Span,
}

#[derive(Debug, Clone)]
pub struct PropertyDef {
	pub attrs: AttrVec,
	pub ident: Ident,
	pub expr: P<Expr>,

	pub id: NodeId,
	pub span: Span,
}
