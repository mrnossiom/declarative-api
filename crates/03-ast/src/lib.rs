#[derive(Debug)]
pub struct Api {
	pub attrs: Vec<Attribute>,
	pub items: Vec<Item>,
	pub span: SpanData,
	pub id: NodeId,
}

#[derive(Debug)]
pub struct Item {
	pub attrs: Vec<Attribute>,
	pub kind: ItemKind,
	pub span: SpanData,
	pub ident: Ident,
	pub id: NodeId,
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
	pub ty: Type,
	pub attrs: Vec<Attribute>,
	pub span: SpanData,
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
	pub span: SpanData,
}

#[derive(Debug)]
pub struct MetaField {
	pub ident: Ident,
	pub value: MetaFieldKind,
	pub span: SpanData,
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
pub struct Type(String);

#[derive(Debug)]
pub struct Attribute {
	pub ident: Ident,
	pub value: String,
	pub span: SpanData,
}

#[derive(Debug)]
pub enum ScopeKind {
	Loaded { items: Vec<Item>, span: SpanData },
	Unloaded,
}

#[derive(Debug)]
pub struct Ident {
	pub symbol: Symbol,
	pub span: SpanData,
}

#[derive(Debug)]
pub struct Symbol(String);
// pub struct Symbol(u32);

#[derive(Debug)]
pub struct SpanData {
	pub start: usize,
	pub end: usize,
}

#[derive(Debug)]
pub struct NodeId(u32);

#[cfg(test)]
mod tests;
