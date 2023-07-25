use std::collections::BTreeSet;

mod types;

pub trait Parse {
	fn parse(
		&self,
		parser_variables: types::ParserVariables,
		intermediate_representation: &mut types::IntermediateRepresentation,
	);
}

impl Parse for ast::types::ApiMetadata {
	fn parse(
		&self,
		_: types::ParserVariables,
		intermediate_representation: &mut types::IntermediateRepresentation,
	) {
		intermediate_representation.metadata.name = self.name.clone().unwrap(); // Panic if this field is not set.
		let version = self.version.as_ref().unwrap(); // Panic if this field is not set.
		intermediate_representation.metadata.version = types::Version {
			major: version.major,
			minor: version.minor,
			patch: version.patch,
		};
		intermediate_representation.metadata.urls = self.urls.clone();
		intermediate_representation.metadata.comment = self.comment.clone();
	}
}

impl Parse for ast::types::Method {
	fn parse(
		&self,
		parser_variables: types::ParserVariables,
		intermediate_representation: &mut types::IntermediateRepresentation,
	) {
		let mut new_parser_variables = parser_variables;
		update_headers(&self.headers, &mut new_parser_variables);
		update_parameters(&self.headers, &mut new_parser_variables);
		intermediate_representation
			.scopes
			.get_mut(&new_parser_variables.scope_path)
			.unwrap()
			.methods
			.insert(types::Method {
				name: self.name.clone(),
				method_path: new_parser_variables.endpoint_path,
				responses: BTreeSet::new(),
				comment: self.comment.clone(),
				headers: new_parser_variables.headers,
				parameters: new_parser_variables.parameters,
				query_params: new_parser_variables.query_params,
			});
	}
}

impl Parse for ast::types::Path {
	fn parse(
		&self,
		parser_variables: types::ParserVariables,
		intermediate_representation: &mut types::IntermediateRepresentation,
	) {
		let mut new_parser_variables = parser_variables;
		new_parser_variables.endpoint_path = new_parser_variables
			.endpoint_path
			.join(self.name.clone())
			.into();
		update_parser_variables(
			&self.headers,
			&self.parameters,
			&self.query,
			&mut new_parser_variables,
		);
		parse_children(
			&self.child_scopes,
			&new_parser_variables,
			intermediate_representation,
		);
		parse_children(
			&self.child_paths,
			&new_parser_variables,
			intermediate_representation,
		);
		parse_children(
			&self.methods,
			&new_parser_variables,
			intermediate_representation,
		);
	}
}

impl Parse for ast::types::Scope {
	fn parse(
		&self,
		parser_variables: types::ParserVariables,
		intermediate_representation: &mut types::IntermediateRepresentation,
	) {
		intermediate_representation
			.scopes
			.get_mut(&parser_variables.scope_path)
			.unwrap()
			.child_scopes
			.insert(self.name.clone());
		let mut new_parser_variables = parser_variables;
		new_parser_variables.scope_path.push(self.name.clone());
		if !(intermediate_representation
			.scopes
			.contains_key(&new_parser_variables.scope_path))
		{
			intermediate_representation
				.scopes
				.insert(new_parser_variables.scope_path.clone(), types::Scope::new());
		}
		update_parser_variables(
			&self.headers,
			&self.parameters,
			&self.query,
			&mut new_parser_variables,
		);
		parse_children(
			&self.child_scopes,
			&new_parser_variables,
			intermediate_representation,
		);
		parse_children(
			&self.child_paths,
			&new_parser_variables,
			intermediate_representation,
		);
		parse_children(
			&self.methods,
			&new_parser_variables,
			intermediate_representation,
		);
	}
}

impl Parse for ast::types::ApiData {
	fn parse(
		&self,
		parser_variables: types::ParserVariables,
		intermediate_representation: &mut types::IntermediateRepresentation,
	) {
		parse_children(
			&self.child_scopes,
			&parser_variables,
			intermediate_representation,
		);
	}
}

impl Parse for ast::types::Api {
	fn parse(
		&self,
		parser_variables: types::ParserVariables,
		intermediate_representation: &mut types::IntermediateRepresentation,
	) {
		self.metadata
			.parse(parser_variables.clone(), intermediate_representation);
		self.data
			.parse(parser_variables, intermediate_representation);
	}
}

fn parse_children<T: Parse>(
	children: &Vec<T>,
	parser_variables: &types::ParserVariables,
	intermediate_representation: &mut types::IntermediateRepresentation,
) {
	for child in children.iter() {
		child.parse(parser_variables.clone(), intermediate_representation);
	}
}

fn update_parser_variables(
	headers: &Vec<ast::types::KeyValuePair>,
	parameters: &Vec<ast::types::KeyValuePair>,
	query_params: &Vec<ast::types::KeyValuePair>,
	parser_variables: &mut types::ParserVariables,
) {
	update_headers(headers, parser_variables);
	update_parameters(parameters, parser_variables);
	update_query(query_params, parser_variables);
}

fn update_headers(
	headers: &Vec<ast::types::KeyValuePair>,
	parser_variables: &mut types::ParserVariables,
) {
	for header in headers.iter() {
		let (key, value) = types::KeyValuePairValue::from_ast_key_value_pair_ref(header);
		parser_variables.headers.insert(key, value);
	}
}

fn update_parameters(
	parameters: &Vec<ast::types::KeyValuePair>,
	parser_variables: &mut types::ParserVariables,
) {
	for parameter in parameters.iter() {
		let (key, value) = types::KeyValuePairValue::from_ast_key_value_pair_ref(parameter);
		parser_variables.parameters.insert(key, value);
	}
}

fn update_query(
	query_params: &Vec<ast::types::KeyValuePair>,
	parser_variables: &mut types::ParserVariables,
) {
	for query_param in query_params.iter() {
		let (key, value) = types::KeyValuePairValue::from_ast_key_value_pair_ref(query_param);
		parser_variables.query_params.insert(key, value);
	}
}

#[cfg(test)]
mod tests {
	use std::{
		collections::{BTreeMap, HashSet},
		path::Path,
		str::FromStr,
	};

	use super::*;

	#[test]
	fn test_1() {
		let v1 = types::Version {
			major: 1,
			minor: 2,
			patch: 3,
		};

		let v2 = types::Version {
			major: 1,
			minor: 2,
			patch: 3,
		};
		assert_eq!(v1, v2);
	}

	#[test]
	fn test_2() {
		let v1 = types::Version {
			major: 1,
			minor: 2,
			patch: 3,
		};
		let v2 = types::Version {
			major: 0,
			minor: 3,
			patch: 4,
		};
		assert!(v1 > v2);
	}

	#[test]
	fn test_3() {
		let input_ast = ast::types::Api {
			metadata: ast::types::ApiMetadata {
				name: Some("Wiro's API".into()),
				version: Some(ast::types::Version {
					major: 1,
					minor: 0,
					patch: 0,
				}),
				urls: vec![],
				comment: Some("Je suis un commentaire de documentation (des métadonnées)".into()),
			},
			data: ast::types::ApiData {
				child_scopes: vec![ast::types::Scope {
					name: "dashboard".into(),
					child_scopes: vec![ast::types::Scope {
						name: "metrics".into(),
						child_scopes: vec![],
						child_models: vec![],
						child_paths: vec![],
						methods: vec![ast::types::Method {
							name: http::Method::from_str("GET").unwrap(),
							responses: vec![],
							headers: vec![],
							query: vec![],
							comment: None,
						}],
						parameters: vec![],
						headers: vec![],
						query: vec![],
						comment: None,
					}],
					child_models: vec![],
					child_paths: vec![],
					methods: vec![],
					parameters: vec![],
					headers: vec![],
					query: vec![],
					comment: None,
				}],
			},
		};
		let mut expected_child_scopes_hashset = HashSet::new();
		expected_child_scopes_hashset.insert("metrics".into());
		let mut expected_methods_hashset = HashSet::new();
		expected_methods_hashset.insert(types::Method {
			name: http::Method::GET,
			responses: BTreeSet::new(),
			comment: None,
			parameters: BTreeMap::new(),
			query_params: BTreeMap::new(),
			headers: BTreeMap::new(),
			method_path: Path::new("/").into(),
		});
		let mut expected_child_scopes_in_root_scope_hashset = HashSet::new();
		expected_child_scopes_in_root_scope_hashset.insert("dashboard".into());
		let mut expected_scopes_hashmap = BTreeMap::new();
		// Root scope
		expected_scopes_hashmap.insert(
			vec![],
			types::Scope {
				child_scopes: expected_child_scopes_in_root_scope_hashset,
				models: HashSet::new(),
				methods: HashSet::new(),
				comment: None,
			},
		);
		// Level 1 scope
		expected_scopes_hashmap.insert(
			vec!["dashboard".into()],
			types::Scope {
				child_scopes: expected_child_scopes_hashset,
				models: HashSet::new(),
				methods: HashSet::new(),
				comment: None,
			},
		);
		// Level 2 scope
		expected_scopes_hashmap.insert(
			vec!["dashboard".into(), "metrics".into()],
			types::Scope {
				child_scopes: HashSet::new(),
				models: HashSet::new(),
				methods: expected_methods_hashset,
				comment: None,
			},
		);
		let expected_models_hashmap = BTreeMap::new();
		let expected_intermediate_representation = types::IntermediateRepresentation {
			metadata: types::ApiMetadata {
				name: "Wiro's API".into(),
				version: types::Version {
					major: 1,
					minor: 0,
					patch: 0,
				},
				urls: vec![],
				comment: Some("Je suis un commentaire de documentation (des métadonnées)".into()),
			},
			scopes: expected_scopes_hashmap,
			models: expected_models_hashmap,
		};
		let mut parsed_intermediate_representation = types::IntermediateRepresentation::new();
		input_ast.parse(
			types::ParserVariables::default(),
			&mut parsed_intermediate_representation,
		);
		//dbg!(&expected_intermediate_representation);
		//dbg!(&parsed_intermediate_representation);
		assert!(expected_intermediate_representation == parsed_intermediate_representation);
	}
}