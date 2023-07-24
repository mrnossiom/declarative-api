use std::{cmp::Ordering, collections::HashMap, fmt};

#[derive(Debug)]
pub struct Version {
	pub major: u32,
	pub minor: u32,
	pub patch: u32,
}

impl fmt::Display for Version {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
	}
}

impl PartialOrd for Version {
	// Required method
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.major
			.partial_cmp(&other.major)
			.and_then(|ord| match ord {
				Ordering::Equal => self.minor.partial_cmp(&other.minor),
				_ => Some(ord),
			})
			.and_then(|ord| match ord {
				Ordering::Equal => self.patch.partial_cmp(&other.patch),
				_ => Some(ord),
			})
	}
}

impl PartialEq for Version {
	fn eq(&self, other: &Self) -> bool {
		self.major == other.major && self.minor == other.minor && self.patch == other.patch
	}
}

#[derive(Debug)]
pub struct ApiMetadata {
	pub name: String,
	pub version: Version,
	pub urls: Vec<String>,
	pub comment: String,
}

#[derive(Debug)]
pub struct Response {
	pub status_code: u32,
	pub body: Vec<KeyValuePair>,
	pub headers: Vec<KeyValuePair>,
	pub comment: String,
}

#[derive(Debug)]
pub struct Method {
	pub responses: Vec<Response>,
	pub headers: Vec<KeyValuePair>,
	pub parameters: Vec<KeyValuePair>,
	pub comment: String,
}

#[derive(Debug)]
pub struct Path {
	pub sub_path: Box<Path>,
	pub methods: Vec<Method>,
	pub headers: Vec<KeyValuePair>,
	pub parameters: Vec<KeyValuePair>,
	pub metadata: Vec<KeyValuePair>,
	pub comment: String,
}

#[derive(Debug)]
pub struct KeyValuePair {
	pub key: String,
	pub type_: Type,
	pub description: String,
	pub parameters: HashMap<String, String>,
	pub comment: String,
}

#[derive(Debug)]
pub struct Scope {
	pub child_scopes: Vec<Scope>,
	pub child_models: Vec<Model>,
	pub child_paths: Vec<Path>,
	pub methods: Vec<Method>,
	pub comment: String,
}

#[derive(Debug)]
pub struct Model {
	pub name: String,
	pub model_body: KeyValuePair,
	pub comment: String,
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

#[derive(Debug)]
pub enum Type {
	Int,

	String,

	Model(String),
	Object(HashMap<String, Type>),
}
