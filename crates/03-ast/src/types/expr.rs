use crate::types::{AttrVec, NodeId};
use crate::P;
use lexer::rich::LiteralKind;
use session::{Ident, Span, Symbol};
use thin_vec::ThinVec;

/// An expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expr {
	pub attrs: AttrVec,
	pub kind: ExprKind,
	// pub tokens: Option<LazyAttrTokenStream>,
	pub id: NodeId,
	pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExprKind {
	/// An array (`[a, b, c, d]`)
	Array(ThinVec<P<Expr>>),
	/// A literal (e.g., `1`, `"foo"`).
	Literal(LiteralKind, Symbol),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropertyDef {
	pub attrs: AttrVec,
	pub ident: Ident,
	pub expr: P<Expr>,

	pub id: NodeId,
	pub span: Span,
}
