#![warn(
	clippy::unwrap_used,
	clippy::nursery,
	clippy::pedantic,
	rustdoc::broken_intra_doc_links
)]

use std::collections::BTreeMap;

use types::ResolveModels;

pub mod types;
pub mod utils;

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

impl Parse for ast::types::Model {
	fn parse(
		&self,
		parser_variables: types::ParserVariables,
		intermediate_representation: &mut types::IntermediateRepresentation,
	) {
		let scope_model_map_ref = &mut intermediate_representation
			.scopes
			.get_mut(&parser_variables.scope_path)
			.expect("tried adding a model to a non-existant scope")
			.models;
		assert!(
			!scope_model_map_ref.contains_key(&self.name),
			"defining \"{:#?}\" multiple times",
			self.name.clone()
		);
		scope_model_map_ref.insert(
			self.name.clone(),
			types::Model {
				model_body: types::KeyValuePairValue::map_from_ast_key_value_pair_vec(
					&self.model_body,
				),
				comment: self.comment.clone(),
			},
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
			&self.child_models,
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
		let cloned_intermediate_representation = intermediate_representation.clone();
		/*for (scope, model) in &mut intermediate_representation.models {
			model
				.model_body
				.resolve_models(scope, cloned_intermediate_representation.clone());
		}*/
		for (scope_path, scope) in &mut intermediate_representation.scopes {
			for endpoint in scope.endpoints.values_mut() {
				for method in endpoint.methods.values_mut() {
					for response in method.responses.values_mut() {
						response
							.body
							.resolve_models(scope_path, cloned_intermediate_representation.clone());
					}
				}
			}
		}
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
mod tests;
