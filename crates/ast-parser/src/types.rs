use std::{
	cmp::Ordering,
	collections::{BTreeMap, BTreeSet, HashSet},
	fmt,
	hash::Hash,
	path::Path,
	vec,
};

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

pub type ScopePath = Vec<String>;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct KeyValuePairValue {
	pub type_: Type,
	pub description: String,
	pub parameters: BTreeMap<String, String>,
	pub comment: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct Method {
	pub name: http::Method,
	pub responses: BTreeSet<Response>,
	pub comment: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct EndpointPath {
	pub methods: Vec<Method>,
	pub parameters: BTreeMap<String, KeyValuePairValue>,
	pub metadata: BTreeMap<String, KeyValuePairValue>,
	pub comment: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct Response {
	pub status_code: u32,
	pub body: BTreeMap<String, KeyValuePairValue>,
	pub headers: BTreeMap<String, KeyValuePairValue>,
	pub comment: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Scope {
	pub child_scopes: HashSet<String>,
	pub models: HashSet<Model>,
	pub methods: HashSet<Method>,
	pub comment: Option<String>,
}

impl Scope {
	pub fn new() -> Scope {
		Scope {
			child_scopes: HashSet::new(),
			models: HashSet::new(),
			methods: HashSet::new(),
			comment: None,
		}
	}
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct Model {
	pub name: String,
	pub model_body: BTreeMap<String, KeyValuePairValue>,
	pub comment: Option<String>,
}

impl Model {
	pub fn new() -> Model {
		Model {
			name: "".into(),
			model_body: BTreeMap::new(),
			comment: None,
		}
	}
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum Type {
	Int,

	String,

	Model(String),
	Object(Box<(String, KeyValuePairValue)>),
}

#[derive(Debug, PartialEq)]
pub struct ApiMetadata {
	pub name: String,
	pub version: Version,
	pub urls: Vec<String>,
	pub comment: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct IntermediateRepresentation {
	pub metadata: ApiMetadata,
	pub scopes: BTreeMap<ScopePath, Scope>,
	pub models: BTreeMap<ScopePath, Model>,
}

impl IntermediateRepresentation {
	pub fn new() -> IntermediateRepresentation {
		let mut scopes_map = BTreeMap::new();
		scopes_map.insert(vec![], Scope::new());
		IntermediateRepresentation {
			metadata: ApiMetadata {
				name: "".into(),
				version: Version {
					major: 0,
					minor: 0,
					patch: 0,
				},
				urls: vec![],
				comment: None,
			},
			scopes: scopes_map,
			models: BTreeMap::new(),
		}
	}
}

#[derive(Debug, Clone)]
pub struct ParserVariables {
	pub scope_path: ScopePath,
	pub endpoint_path: Box<Path>,
	pub headers: BTreeMap<String, KeyValuePairValue>,
	pub parameters: BTreeMap<String, KeyValuePairValue>,
	pub comment: Option<String>,
}

impl ParserVariables {
	pub fn new() -> ParserVariables {
		ParserVariables {
			scope_path: ScopePath::new(),
			endpoint_path: Path::new("/").into(),
			headers: BTreeMap::new(),
			parameters: BTreeMap::new(),
			comment: None,
		}
	}
}
