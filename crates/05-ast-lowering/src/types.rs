use std::{
	cmp::Ordering,
	collections::{BTreeMap, HashMap, HashSet},
	fmt,
	hash::Hash,
	path::Path,
};

#[derive(Debug, Clone)]
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

pub trait ResolveModels {
	fn resolve_models(
		&mut self,
		scope: &ScopePath,
		intermediate_representation: IntermediateRepresentation,
	);
}

pub type ScopePath = Vec<String>;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct KeyValuePairValue {
	pub type_: Type,
	pub description: String,
	pub parameters: KeyValuePairValueParameters,
	pub comment: Option<String>,
}

impl KeyValuePairValue {
	#[must_use]
	pub fn from_ast_key_value_pair_ref(
		key_value_pair: &ast::types::KeyValuePair,
	) -> (String, Self) {
		(
			key_value_pair.key.clone(),
			Self {
				type_: Type::from_ast_type(key_value_pair.type_.clone()),
				description: key_value_pair.description.clone(),
				parameters: Self::parse_parameters(&key_value_pair.parameters),
				comment: key_value_pair.comment.clone(),
			},
		)
	}
	#[must_use]
	#[allow(clippy::missing_panics_doc)]
	pub fn map_from_ast_key_value_pair_vec(
		key_value_pair_vec: &[ast::types::KeyValuePair],
	) -> BTreeMap<String, Self> {
		let mut btreemap = BTreeMap::new();
		for field in key_value_pair_vec {
			let param_btreemap = Self::parse_parameters(&field.parameters);
			assert!(
				!btreemap.contains_key(&field.key),
				"a key-value pair has the same key defined multiple times"
			);
			btreemap.insert(
				field.key.clone(),
				Self {
					type_: Type::from_ast_type(field.type_.clone()),
					description: field.description.clone(),
					parameters: param_btreemap,
					comment: field.comment.clone(),
				},
			);
		}
		btreemap
	}
	// This merges the b BTreeMap into the a BTreeMap. In case of the same keys being defined in both BTreeMap, b's values has priority.
	pub fn merge(a: &mut BTreeMap<String, Self>, b: &BTreeMap<String, Self>) {
		for (key, value) in b {
			a.insert(key.clone(), value.clone());
		}
	}
	#[must_use]
	#[allow(clippy::missing_panics_doc)]
	pub fn parse_parameters(
		parameters: &[ast::types::KeyValuePairParameter],
	) -> BTreeMap<String, String> {
		let mut param_btreemap = BTreeMap::new();
		for parameter in parameters {
			assert!(
				!param_btreemap.contains_key(&parameter.0.clone()),
				"a key-value pair has the same parameter defined multiple times"
			);
			param_btreemap.insert(parameter.0.clone(), parameter.1.clone());
		}
		param_btreemap
	}
}

impl ResolveModels for BTreeMap<String, KeyValuePairValue> {
	fn resolve_models(
		&mut self,
		scope: &ScopePath,
		intermediate_representation: IntermediateRepresentation,
	) {
		for value in self.values_mut() {
			match &mut value.type_ {
				Type::Model(model_name) => {
					let mut found = false;
					let mut current_scope = scope.clone();
					while !found {
						found = intermediate_representation
							.scopes
							.get(&current_scope)
							.expect("missing scope")
							.models
							.contains_key(model_name);
						if !found {
							assert!(
								!current_scope.is_empty(),
								"unknown reference to model \"{model_name}\""
							);
							current_scope.truncate(current_scope.len() - 1);
						}
					}
					current_scope.push(model_name.clone());
					value.type_ = Type::ResolvedModel(current_scope);
				}
				Type::Object(child_object) => {
					child_object.resolve_models(scope, intermediate_representation.clone());
				}
				_ => (),
			};
		}
	}
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct Method {
	pub parameters: BTreeMap<String, KeyValuePairValue>,
	pub headers: BTreeMap<String, KeyValuePairValue>,
	pub query_params: BTreeMap<String, KeyValuePairValue>,
	pub responses: BTreeMap<u32, Response>,
	pub comment: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct Response {
	pub body: BTreeMap<String, KeyValuePairValue>,
	pub headers: BTreeMap<String, KeyValuePairValue>,
	pub comment: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, Default)]
pub struct Endpoint {
	pub methods: BTreeMap<String, Method>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Scope {
	pub child_scopes: HashSet<String>,
	pub models: BTreeMap<String, Model>,
	pub endpoints: HashMap<String, Endpoint>,
	pub comment: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, Default)]
pub struct Model {
	pub model_body: BTreeMap<String, KeyValuePairValue>,
	pub comment: Option<String>,
}

pub type KeyValuePairValueParameters = BTreeMap<String, String>;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum Type {
	Int,

	String,

	Model(String),
	ResolvedModel(ScopePath),
	Object(BTreeMap<String, KeyValuePairValue>),
	List(Vec<Type>),
}

impl Type {
	#[must_use]
	pub fn from_ast_type(input: ast::types::Type) -> Self {
		match input {
			ast::types::Type::Int => Self::Int,
			ast::types::Type::String => Self::String,
			ast::types::Type::Model(model_name) => Self::Model(model_name),
			ast::types::Type::Object(fields) => {
				Self::Object(KeyValuePairValue::map_from_ast_key_value_pair_vec(&fields))
			}
			ast::types::Type::List(list_obj) => {
				let mut out_list_obj = vec![];
				for obj in &list_obj {
					out_list_obj.push(Self::from_ast_type(obj.clone()));
				}
				Self::List(out_list_obj)
			}
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
pub struct ApiMetadata {
	pub name: String,
	pub version: Version,
	pub urls: Vec<String>,
	pub comment: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IntermediateRepresentation {
	pub metadata: ApiMetadata,
	pub scopes: BTreeMap<ScopePath, Scope>,
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
		}
	}
}

#[derive(Debug, Clone)]
pub struct ParserVariables {
	pub current_method: String,
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
			current_method: String::new(),
			scope_path: ScopePath::new(),
			endpoint_path: Path::new("/").into(),
			headers: BTreeMap::new(),
			parameters: BTreeMap::new(),
			query_params: BTreeMap::new(),
			comment: None,
		}
	}
}
