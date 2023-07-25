use std::{
	cmp::Ordering,
	collections::{BTreeMap, BTreeSet, HashMap, HashSet},
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

impl KeyValuePairValue {
	pub fn from_ast_key_value_pair_ref(
		key_value_pair: &ast::types::KeyValuePair,
	) -> (String, Self) {
		(
			key_value_pair.key.clone(),
			Self {
				type_: Type::from_ast_type(key_value_pair.type_.clone()),
				description: key_value_pair.description.clone(),
				parameters: key_value_pair.parameters.clone(),
				comment: key_value_pair.comment.clone(),
			},
		)
	}
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct Method {
	pub name: http::Method,
	pub parameters: BTreeMap<String, KeyValuePairValue>,
	pub headers: BTreeMap<String, KeyValuePairValue>,
	pub query_params: BTreeMap<String, KeyValuePairValue>,
	pub responses: BTreeSet<Response>,
	pub comment: Option<String>,
}

impl Ord for Method {
	fn cmp(&self, other: &Self) -> Ordering {
		self.name.to_string().cmp(&other.name.to_string())
	}
}

impl PartialOrd for Method {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.name.to_string().cmp(&other.name.to_string()))
	}
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct Response {
	pub status_code: u32,
	pub body: BTreeMap<String, KeyValuePairValue>,
	pub comment: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, Default)]
pub struct Endpoint {
	pub methods: BTreeSet<Method>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Scope {
	pub child_scopes: HashSet<String>,
	pub models: HashSet<Model>,
	pub endpoints: HashMap<String, Endpoint>,
	pub comment: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, Default)]
pub struct Model {
	pub name: String,
	pub model_body: BTreeMap<String, KeyValuePairValue>,
	pub comment: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum Type {
	Int,

	String,

	Model(String),
	Object(Box<(String, KeyValuePairValue)>),
}

impl Type {
	pub fn from_ast_type(input: ast::types::Type) -> Self {
		match input {
			ast::types::Type::Int => Self::Int,
			ast::types::Type::String => Self::String,
			ast::types::Type::Model(model_name) => Self::Model(model_name),
			ast::types::Type::Object(object) => Self::Object(Box::new((
				object.key,
				KeyValuePairValue {
					type_: Self::from_ast_type(object.type_),
					description: object.description,
					parameters: object.parameters,
					comment: object.comment,
				},
			))),
		}
	}
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

impl Default for IntermediateRepresentation {
	fn default() -> Self {
		let mut scopes_map = BTreeMap::new();
		scopes_map.insert(vec![], Scope::default());
		Self {
			metadata: ApiMetadata {
				name: String::new(),
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
	pub query_params: BTreeMap<String, KeyValuePairValue>,
	pub comment: Option<String>,
}

impl Default for ParserVariables {
	fn default() -> Self {
		Self {
			scope_path: ScopePath::new(),
			endpoint_path: Path::new("/").into(),
			headers: BTreeMap::new(),
			parameters: BTreeMap::new(),
			query_params: BTreeMap::new(),
			comment: None,
		}
	}
}
