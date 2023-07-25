use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Version {
	pub major: u32,
	pub minor: u32,
	pub patch: u32,
}

#[derive(Debug)]
pub struct ApiMetadata {
	pub name: Option<String>,
	pub version: Option<Version>,
	pub urls: Vec<String>,
	pub comment: Option<String>,
}

#[derive(Debug)]
pub struct Response {
	pub status_code: u32,
	pub body: Vec<KeyValuePair>,
	pub headers: Vec<KeyValuePair>,
	pub comment: Option<String>,
}

#[derive(Debug)]
pub struct Method {
	pub name: http::Method,
	pub responses: Vec<Response>,
	pub headers: Vec<KeyValuePair>,
	pub query: Vec<KeyValuePair>,
	pub comment: Option<String>,
}

#[derive(Debug)]
pub struct Path {
	pub name: String,
	pub child_scopes: Vec<Scope>,
	pub child_paths: Vec<Path>,
	pub methods: Vec<Method>,
	pub headers: Vec<KeyValuePair>,
	pub parameters: Vec<KeyValuePair>,
	pub query: Vec<KeyValuePair>,
	pub metadata: Vec<KeyValuePair>,
	pub comment: Option<String>,
}

#[derive(Debug, Clone)]
pub struct KeyValuePair {
	pub key: String,
	pub type_: Type,
	pub description: String,
	pub parameters: BTreeMap<String, String>,
	pub comment: Option<String>,
}

#[derive(Debug)]
pub struct Scope {
	pub name: String,
	pub child_scopes: Vec<Scope>,
	pub child_models: Vec<Model>,
	pub child_paths: Vec<Path>,
	pub methods: Vec<Method>,
	pub headers: Vec<KeyValuePair>,
	pub parameters: Vec<KeyValuePair>,
	pub query: Vec<KeyValuePair>,
	pub comment: Option<String>,
}

#[derive(Debug)]
pub struct Model {
	pub name: String,
	pub model_body: Vec<KeyValuePair>,
	pub comment: Option<String>,
}

#[derive(Debug)]
pub struct ApiData {
	pub child_scopes: Vec<Scope>,
}

#[derive(Debug)]
pub struct Api {
	pub metadata: ApiMetadata,
	pub data: ApiData,
}

#[derive(Debug, Clone)]
pub enum Type {
	Int,

	String,

	Model(String),
	Object(Box<KeyValuePair>),
}
