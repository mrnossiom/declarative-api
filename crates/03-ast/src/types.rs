use lexer::{span::Span, symbols::Symbol};

#[derive(Debug)]
pub struct Api {
	pub attrs: Vec<Attribute>,
	pub meta: Metadata,
	pub items: Vec<Item>,
	pub span: Span,
}

#[derive(Debug)]
pub struct Metadata {
	pub name: String,
	pub description: String,
	pub licence: String,
	pub urls: Vec<String>,
}

#[derive(Debug)]
pub struct Item {
	pub attrs: Vec<Attribute>,
	pub kind: ItemKind,
	pub span: Span,
	pub ident: Ident,
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
pub struct Type(pub(crate) String);

#[derive(Debug)]
pub struct Attribute {
	pub ident: Ident,
	pub value: String,
	pub span: Span,
}

#[derive(Debug)]
pub enum ScopeKind {
	Loaded {
		items: Vec<Item>,
		/// Whether the scope was defined inline or in an external file.
		inline: bool,
		span: Span,
	},
	Unloaded,
}

#[derive(Debug)]
pub struct Ident {
	pub symbol: Symbol,
	pub span: Span,
}
