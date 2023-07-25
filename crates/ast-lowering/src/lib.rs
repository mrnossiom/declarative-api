#![warn(
	clippy::unwrap_used,
	clippy::nursery,
	clippy::pedantic,
	rustdoc::broken_intra_doc_links
)]

use std::collections::BTreeMap;

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
		intermediate_representation.metadata.name =
			self.name.clone().expect("please provide an API name"); // Panic if this field is not set.
		let version = self
			.version
			.as_ref()
			.expect("please provide an API version"); // Panic if this field is not set.
		intermediate_representation.metadata.version = types::Version {
			major: version.major,
			minor: version.minor,
			patch: version.patch,
		};
		intermediate_representation.metadata.urls = self.urls.clone();
		intermediate_representation.metadata.comment = self.comment.clone();
	}
}

impl Parse for ast::types::Response {
	fn parse(
		&self,
		parser_variables: types::ParserVariables,
		intermediate_representation: &mut types::IntermediateRepresentation,
	) {
		let endpoint_path = parser_variables
			.endpoint_path
			.to_str()
			.expect("failed to generate endpoint path")
			.to_string();
		let current_scope: &mut types::Scope = intermediate_representation
			.scopes
			.get_mut(&parser_variables.scope_path)
			.expect("tried adding an endpoint to a non-existant scope");
		let method_responses = &mut current_scope
			.endpoints
			.get_mut(&endpoint_path)
			.expect("tried adding a method to a non-existant endpoint")
			.methods
			.get_mut(&parser_variables.current_method)
			.expect("tried adding a response to a non-existant endpoint")
			.responses;
		assert!(
			!method_responses.contains_key(&self.status_code),
			"a response was defined twice"
		);
		method_responses.insert(
			self.status_code,
			types::Response {
				headers: types::KeyValuePairValue::map_from_ast_key_value_pair_vec(&self.headers),
				body: types::KeyValuePairValue::map_from_ast_key_value_pair_vec(&self.body),
				comment: self.comment.clone(),
			},
		);
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
		update_query(&self.query, &mut new_parser_variables);
		let endpoint_path = new_parser_variables
			.endpoint_path
			.to_str()
			.expect("failed to generate endpoint path")
			.to_string();
		let current_scope = intermediate_representation
			.scopes
			.get_mut(&new_parser_variables.scope_path)
			.expect("tried adding an endpoint to a non-existant scope");
		if !(current_scope.endpoints.contains_key(&endpoint_path)) {
			current_scope
				.endpoints
				.insert(endpoint_path.clone(), types::Endpoint::default());
		}
		let endpoint_methods = &mut current_scope
			.endpoints
			.get_mut(&endpoint_path)
			.expect("tried adding a method to a non-existant endpoint")
			.methods;
		let method_string = self.method.to_string();
		assert!(
			!endpoint_methods.contains_key(&method_string),
			"a method was defined twice"
		);
		endpoint_methods.insert(
			method_string.clone(),
			types::Method {
				responses: BTreeMap::new(),
				comment: self.comment.clone(),
				headers: new_parser_variables.headers.clone(),
				parameters: new_parser_variables.parameters.clone(),
				query_params: new_parser_variables.query_params.clone(),
			},
		);
		new_parser_variables.current_method = method_string;
		parse_children(
			&self.responses,
			&new_parser_variables,
			intermediate_representation,
		);
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
			.expect("tried adding a child scope to a non-existant scope")
			.child_scopes
			.insert(self.name.clone());
		let mut new_parser_variables = parser_variables;
		new_parser_variables.scope_path.push(self.name.clone());
		if !(intermediate_representation
			.scopes
			.contains_key(&new_parser_variables.scope_path))
		{
			intermediate_representation.scopes.insert(
				new_parser_variables.scope_path.clone(),
				types::Scope::default(),
			);
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
	children: &[T],
	parser_variables: &types::ParserVariables,
	intermediate_representation: &mut types::IntermediateRepresentation,
) {
	for child in children.iter() {
		child.parse(parser_variables.clone(), intermediate_representation);
	}
}

fn update_parser_variables(
	headers: &[ast::types::KeyValuePair],
	parameters: &[ast::types::KeyValuePair],
	query_params: &[ast::types::KeyValuePair],
	parser_variables: &mut types::ParserVariables,
) {
	update_headers(headers, parser_variables);
	update_parameters(parameters, parser_variables);
	update_query(query_params, parser_variables);
}

fn update_headers(
	headers: &[ast::types::KeyValuePair],
	parser_variables: &mut types::ParserVariables,
) {
	types::KeyValuePairValue::merge(
		&mut parser_variables.headers,
		&types::KeyValuePairValue::map_from_ast_key_value_pair_vec(headers),
	);
}

fn update_parameters(
	parameters: &[ast::types::KeyValuePair],
	parser_variables: &mut types::ParserVariables,
) {
	types::KeyValuePairValue::merge(
		&mut parser_variables.parameters.clone(),
		&types::KeyValuePairValue::map_from_ast_key_value_pair_vec(parameters),
	);
}

fn update_query(
	query_params: &[ast::types::KeyValuePair],
	parser_variables: &mut types::ParserVariables,
) {
	types::KeyValuePairValue::merge(
		&mut parser_variables.query_params.clone(),
		&types::KeyValuePairValue::map_from_ast_key_value_pair_vec(query_params),
	);
}

#[cfg(test)]
mod tests {
	use std::{
		collections::{BTreeMap, HashMap, HashSet},
		str::FromStr,
		vec,
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
	#[allow(clippy::too_many_lines)]
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
					child_scopes: vec![],
					child_models: vec![],
					child_paths: vec![ast::types::Path {
						name: "dashboard".into(),
						child_scopes: vec![],
						child_paths: vec![ast::types::Path {
							name: "metrics".into(),
							child_paths: vec![],
							comment: None,
							methods: vec![],
							headers: vec![
								ast::types::KeyValuePair {
									key: "Authorization".into(),
									type_: ast::types::Type::String,
									description: "The authorization key.".into(),
									parameters: vec![],
									comment: None,
								}
							],
							metadata: vec![],
							parameters: vec![],
							query: vec![],
							child_scopes: vec![ast::types::Scope {
								name: "metrics".into(),
								child_scopes: vec![],
								child_models: vec![],
								child_paths: vec![],
								methods: vec![ast::types::Method {
									method: http::Method::from_str("GET").expect(
												"a method used for testing the ast-lowering crate doesn't exist",
											),
											responses: vec![
												ast::types::Response{
													headers: vec![],
													status_code: 200,
													body:vec![ast::types::KeyValuePair{
														key:"status".into(),
														type_:ast::types::Type::Object(
															vec![
																ast::types::KeyValuePair{
																	key:"message".into(),
																	type_:ast::types::Type::String,
																	description:"The status message itself".into(),
																	parameters:vec![],
																	comment: None,
																},ast::types::KeyValuePair{
																	key:"code".into(),
																	type_:ast::types::Type::Int,
																	description:"The status code".into(),
																	parameters:vec![],
																	comment: None,
																}
															]
														),
														description:"A status message container".into(),
														parameters:vec![],
														comment: None,
													}],
													comment: None
												}
											],
											headers: vec![],
											query: vec![],
											comment: None,
										}],
								parameters: vec![],
								headers: vec![],
								query: vec![],
								comment: None,
							}],
						}],
						methods: vec![],
						headers: vec![],
						parameters: vec![],
						query: vec![],
						metadata: vec![],
						comment: None,
					}],
					methods: vec![],
					parameters: vec![],
					headers: vec![],
					query: vec![],
					comment: None,
				}],
			},
		};
		let mut expected_endpoint_object = types::Endpoint::default();
		expected_endpoint_object.methods.insert(
			http::Method::GET.to_string(),
			types::Method {
				responses: BTreeMap::from([(
					200,
					types::Response {
						body: BTreeMap::from([(
							"status".into(),
							types::KeyValuePairValue {
								type_: types::Type::Object(BTreeMap::from([
									(
										"code".into(),
										types::KeyValuePairValue {
											type_: types::Type::Int,
											description: "The status code".into(),
											parameters: BTreeMap::new(),
											comment: None,
										},
									),
									(
										"message".into(),
										types::KeyValuePairValue {
											type_: types::Type::String,
											description: "The status message itself".into(),
											parameters: BTreeMap::new(),
											comment: None,
										},
									),
								])),
								description: "A status message container".into(),
								parameters: BTreeMap::new(),
								comment: None,
							},
						)]),
						headers: BTreeMap::new(),
						comment: None,
					},
				)]),
				comment: None,
				parameters: BTreeMap::new(),
				query_params: BTreeMap::new(),
				headers: BTreeMap::from([(
					"Authorization".into(),
					types::KeyValuePairValue {
						type_: types::Type::String,
						description: "The authorization key.".into(),
						parameters: BTreeMap::new(),
						comment: None,
					},
				)]),
			},
		);
		let expected_scopes_btreemap = BTreeMap::from([
			// Root scope
			(
				vec![],
				types::Scope {
					child_scopes: HashSet::from(["dashboard".into()]),
					models: HashSet::new(),
					endpoints: HashMap::new(),
					comment: None,
				},
			),
			// Level 1 scope
			(
				vec!["dashboard".into()],
				types::Scope {
					child_scopes: HashSet::from(["metrics".into()]),
					models: HashSet::new(),
					endpoints: HashMap::new(),
					comment: None,
				},
			),
			// Level 2 scope
			(
				vec!["dashboard".into(), "metrics".into()],
				types::Scope {
					child_scopes: HashSet::new(),
					models: HashSet::new(),
					endpoints: HashMap::from([(
						"/dashboard/metrics".into(),
						expected_endpoint_object,
					)]),
					comment: None,
				},
			),
		]);
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
			scopes: expected_scopes_btreemap,
			models: BTreeMap::new(),
		};
		let mut parsed_intermediate_representation = types::IntermediateRepresentation::default();
		input_ast.parse(
			types::ParserVariables::default(),
			&mut parsed_intermediate_representation,
		);
		// dbg!(&expected_intermediate_representation);
		// dbg!(&parsed_intermediate_representation);
		assert!(expected_intermediate_representation == parsed_intermediate_representation);
	}
}
