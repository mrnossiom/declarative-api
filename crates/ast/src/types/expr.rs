use crate::types::{AttrVec, FieldDef, NodeId, P};
use dapic_lexer::rich::LiteralKind;
use dapic_session::{Ident, Span, Symbol};
use thin_vec::ThinVec;

/// An expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expr {
	pub attrs: AttrVec,
	pub kind: ExprKind,
	pub id: NodeId,
	pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExprKind {
	// -- Bases --
	/// A literal (e.g., `1`, `"foo"`).
	Literal(LiteralKind, Symbol),
	/// A path (`path::to::model::Type`).
	Path(Path),
	// TODO: discuss whether this is a bad idea, not sure to implement it
	/// A template for status codes (e.g., `~2xx`).
	Template(()),

	// -- Composables --
	/// An array (`[a, b, c, d]`)
	Array(ThinVec<P<Expr>>),
	/// Access of a named (e.g., `obj.foo`) field.
	Field(P<Expr>, Ident),
}

/// A "Path" is essentially a name.
///
/// It's represented as a sequence of identifiers.
///
/// E.g., `path::to::model::Type`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path {
	/// The segments in the path: the things separated by `::`.
	pub segments: ThinVec<PathSegment>,

	pub span: Span,
}

/// A segment of a path: an identifier.
///
/// E.g., `user` or `Type`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathSegment {
	pub ident: Ident,
	pub id: NodeId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropertyDef {
	pub attrs: AttrVec,
	pub ident: Ident,
	pub expr: P<Expr>,

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

	/// A type surrounded with parentheses
	/// e.g. `(Ty)`
	Paren(P<Ty>),

	/// A model defined inlined
	/// e.g. `{ error string }`
	InlineModel(ThinVec<P<FieldDef>>),
}
