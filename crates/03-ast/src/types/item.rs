use crate::types::{AttrVec, NodeId, PropertyDef, Ty};
use crate::P;
use session::{Ident, Span};
use thin_vec::ThinVec;

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

	Scope(ScopeKind),
	Path(PathItem),
	Model(Model),
	Query(Query),
	Headers(Headers),
	Verb(Verb),
	StatusCode(StatusCode),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verb {
	pub method: Ident,
	pub items: ThinVec<P<Item>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusCode {
	pub code: u16,
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
