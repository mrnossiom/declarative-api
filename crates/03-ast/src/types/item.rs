use super::{AttrVec, Ident, NodeId, PropertyDef, Type};
use crate::P;
use lexer::span::Span;
use thin_vec::ThinVec;

#[derive(Debug, Clone)]
pub struct Item {
	pub attrs: AttrVec,
	pub kind: ItemKind,
	pub ident: Ident,

	pub id: NodeId,
	pub span: Span,
}

#[derive(Debug)]
pub enum ItemKind {
	Meta(Meta),

	Scope(ScopeKind),
	Path(Path),
	Model(Model),
	Headers(Headers),
	Verb(Verb),
	StatusCode(StatusCode),
}

#[derive(Debug)]
pub struct Verb {
	pub method: String,
	pub items: Vec<Item>,
}

#[derive(Debug)]
pub struct StatusCode {
	pub code: u16,
	pub items: Vec<Item>,
}

#[derive(Debug)]
pub struct Meta {
	pub fields: Vec<MetaField>,
}

#[derive(Debug)]
pub struct Headers {
	pub headers: Vec<HeaderField>,
}

#[derive(Debug)]
pub struct HeaderField {
	pub ident: Ident,
	pub attrs: Vec<Attribute>,
	pub span: Span,
}

#[derive(Debug)]
pub struct Path {
	pub path: PathKind,
	pub items: Vec<Item>,
}

#[derive(Debug)]
pub enum PathKind {
	String(Ident),
	Variable(Ident),
	Complex(Vec<Self>),
}

#[derive(Debug)]
pub struct Model {
	pub fields: Vec<ModelField>,
}
#[derive(Debug)]
pub struct ModelField {
	pub ident: Ident,
	pub ty: Type,
	pub attrs: Vec<Attribute>,
	pub span: Span,
}

#[derive(Debug)]
pub struct MetaField {
	pub ident: Ident,
	pub value: MetaFieldKind,
	pub span: Span,
}

#[derive(Debug)]
pub enum MetaFieldKind {
	Str(String),
	Bool(bool),
	Int(u64),
	Float(f64),
	Vec(Vec<Self>),
}

#[derive(Debug)]
pub enum ScopeKind {
	Loaded {
		items: Vec<P<Item>>,
		/// Whether the scope was defined inline or in an external file.
		inline: bool,
		span: Span,
	},
	Unloaded,
}

/// Field definition in a struct, variant or union.
///
/// E.g., `bar: usize` as in `struct Foo { bar: usize }`.
#[derive(Clone, Debug)]
pub struct FieldDef {
	pub attrs: AttrVec,
	pub ident: Option<Ident>,
	pub ty: P<Type>,

	pub id: NodeId,
	pub span: Span,
}
