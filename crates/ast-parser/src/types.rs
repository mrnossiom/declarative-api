use std::{cmp::Ordering, collections::HashMap, fmt, vec};

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct ScopePath {
	path: Vec<String>,
}

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

#[derive(Debug, PartialEq)]
pub struct KeyValuePairValue {
	pub type_: Type,
	pub description: String,
	pub parameters: HashMap<String, String>,
	pub comment: String,
}

#[derive(Debug, PartialEq)]
pub struct Method {
	pub responses: Vec<Response>,
	pub headers: HashMap<String, KeyValuePairValue>,
	pub parameters: HashMap<String, KeyValuePairValue>,
	pub comment: String,
}

#[derive(Debug, PartialEq)]
pub struct Path {
	pub methods: Vec<Method>,
	pub headers: HashMap<String, KeyValuePairValue>,
	pub parameters: HashMap<String, KeyValuePairValue>,
	pub metadata: HashMap<String, KeyValuePairValue>,
	pub comment: String,
}

#[derive(Debug, PartialEq)]
pub struct Response {
	pub status_code: u32,
	pub body: HashMap<String, KeyValuePairValue>,
	pub headers: HashMap<String, KeyValuePairValue>,
	pub comment: String,
}

#[derive(Debug, PartialEq)]
pub struct Scope {
	pub child_scopes: Vec<Scope>,
	pub child_models: Vec<Model>,
	pub child_paths: Vec<Path>,
	pub methods: Vec<Method>,
	pub comment: String,
}

#[derive(Debug, PartialEq)]
pub struct Model {
	pub name: String,
	pub model_body: HashMap<String, KeyValuePairValue>,
	pub comment: String,
}

#[derive(Debug, PartialEq)]
pub enum Type {
	Int,

	String,

	Model(String),
	Object(Box<(String, KeyValuePairValue)>),
}

#[derive(Debug, PartialEq)]
pub struct ApiMetadata {
	pub name: Option<String>,
	pub version: Option<Version>,
	pub urls: Vec<String>,
	pub comment: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct IntermediateRepresentation {
	pub metadata: ApiMetadata,
	pub scopes: HashMap<ScopePath, Scope>,
}

impl IntermediateRepresentation {
	pub fn new() -> IntermediateRepresentation {
		IntermediateRepresentation {
			metadata: ApiMetadata {
				name: None,
				version: None,
				urls: vec![],
				comment: None,
			},
			scopes: HashMap::new(),
		}
	}
}

#[derive(Debug)]
pub struct ParserVariables {
	pub scope_path: ScopePath,
	pub endpoint_path: String,
	pub headers: HashMap<String, KeyValuePairValue>,
	pub parameters: HashMap<String, KeyValuePairValue>,
	pub comment: String,
}

impl ParserVariables {
	pub fn new() -> ParserVariables {
		ParserVariables {
			scope_path: ScopePath { path: vec![] },
			endpoint_path: "".into(),
			headers: HashMap::new(),
			parameters: HashMap::new(),
			comment: "".into(),
		}
	}
}
