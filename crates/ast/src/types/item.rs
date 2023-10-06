use crate::types::{AttrVec, NodeId, PropertyDef, Ty, P};
use session::{Ident, Span};
use thin_vec::ThinVec;

use super::Expr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
	pub attrs: AttrVec,
	pub kind: ItemKind,
	pub ident: Ident,

	pub id: NodeId,
	pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemKind {
	Meta(Metadata),

	Auth(Auth),
	Scope(ScopeKind),
	Path(PathItem),
	Model(Model),
	Enum(Enum),
	Query(Query),
	Headers(Headers),
	Verb(Verb),
	StatusCode(StatusCode),
	Body(Body),
	Params(Params),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Auth {
	Use,
	Define {
		// TODO
	},
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Body {
	pub ty: P<Ty>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Params {
	pub properties: ThinVec<P<FieldDef>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verb {
	// TODO: change to custom verb type
	pub method: Ident,
	pub items: ThinVec<P<Item>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusCode {
	// TODO: change for more advanced expression
	// e.g.
	// ```dapi
	// code 200 {}
	// code 5xx {}
	// code IM_A_TEAPOT {}
	// ```
	pub code: P<Expr>,
	pub items: ThinVec<P<Item>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Contains information like name, description, licence or base server urls.
pub struct Metadata {
	pub fields: ThinVec<P<PropertyDef>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Headers {
	pub headers: ThinVec<P<FieldDef>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Model {
	pub fields: ThinVec<P<FieldDef>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enum {
	pub variants: ThinVec<P<PropertyDef>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Query {
	pub fields: ThinVec<P<FieldDef>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathItem {
	pub kind: PathKind,
	pub items: ThinVec<P<Item>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathKind {
	Simple(Ident),
	Variable(Ident),
	Complex(ThinVec<Self>),
	Current,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScopeKind {
	Loaded {
		items: ThinVec<P<Item>>,
		/// Whether the scope was defined inline or in an external file.
		inline: bool,
		span: Span,
	},
	Unloaded,
}

/// Field definition in a struct, variant or union.
///
/// E.g., `bar: usize` as in `struct Foo { bar: usize }`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldDef {
	pub attrs: AttrVec,
	pub ident: Ident,
	pub ty: P<Ty>,

	pub id: NodeId,
	pub span: Span,
}
