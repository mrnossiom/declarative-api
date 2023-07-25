
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
			urls: vec![
				"https://paradigm.lighton.ai/api/v1".into(),
				"https://paradigm-preprod.lighton.ai/api/v1".into(),
				"https://paradigm-dev.lighton.ai/api/v1".into(),
			],
			comment: Some("Je suis un commentaire de documentation (des métadonnées)".into()),
		},
		data: ast::types::ApiData {
			child_scopes: vec![ast::types::Scope {
				name: "dashboard".into(),
				child_scopes: vec![],
				child_models: vec![ast::types::Model{
					name:"metrics".into(),
					model_body: vec![ast::types::KeyValuePair{
						key:"name".into(),
						type_:ast::types::Type::String,
						description:"The name of the metric".into(),
						parameters:vec![],
						comment:None,
					},ast::types::KeyValuePair{
						key:"email".into(),
						type_:ast::types::Type::String,
						description:String::new(),
						parameters:vec![],
						comment:None,
					},ast::types::KeyValuePair{
						key:"password".into(),
						type_:ast::types::Type::String,
						description:String::new(),
						parameters:vec![],
						comment:None,
					},
					],
				comment:None,
				}],
				child_paths: vec![ast::types::Path {
					name: "dashboard".into(),
					child_scopes: vec![],
					child_paths: vec![ast::types::Path {
						name: "metrics".into(),
						child_paths: vec![],
						comment: None,
						methods: vec![],
						headers: vec![ast::types::KeyValuePair {
							key: "Authorization".into(),
							type_: ast::types::Type::String,
							description: "The authorization key.".into(),
							parameters: vec![],
							comment: None,
						}],
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
													body:vec![
                                                        
                                                    ast::types::KeyValuePair{
														key:"metrics".into(),
														type_:ast::types::Type::Model("metrics".into()),
														description:String::new(),
														parameters:vec![],
														comment: None,
													},ast::types::KeyValuePair{
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
						"metrics".into(),
						types::KeyValuePairValue {
							type_: types::Type::ResolvedModel(vec!["dashboard".into(),"metrics".into()]),
							description: String::new(),
							parameters: BTreeMap::new(),
							comment: None,
						},
					),(
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
				models: BTreeMap::new(),
				endpoints: HashMap::new(),
				comment: None,
			},
		),
		// Level 1 scope
		(
			vec!["dashboard".into()],
			types::Scope {
				child_scopes: HashSet::from(["metrics".into()]),
				models: BTreeMap::from(
					[("metrics".into(),types::Model{
						model_body: BTreeMap::from(
							[("name".into(),types::KeyValuePairValue{
								type_:types::Type::String,
								description:"The name of the metric".into(),
								parameters: BTreeMap::new(),
								comment:None,
							}),("email".into(),types::KeyValuePairValue{
								type_:types::Type::String,
								description:String::new(),
								parameters: BTreeMap::new(),
								comment:None,
							}),("password".into(),types::KeyValuePairValue{
								type_:types::Type::String,
								description:String::new(),
								parameters: BTreeMap::new(),
								comment:None,
							})]
						),
						comment: None,
					})]
				),
				endpoints: HashMap::new(),
				comment: None,
			},
		),
		// Level 2 scope
		(
			vec!["dashboard".into(), "metrics".into()],
			types::Scope {
				child_scopes: HashSet::new(),
				models: BTreeMap::new(),
				endpoints: HashMap::from([("/dashboard/metrics".into(), expected_endpoint_object)]),
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
			urls: vec![
				"https://paradigm.lighton.ai/api/v1".into(),
				"https://paradigm-preprod.lighton.ai/api/v1".into(),
				"https://paradigm-dev.lighton.ai/api/v1".into(),
			],
			comment: Some("Je suis un commentaire de documentation (des métadonnées)".into()),
		},
		models: BTreeMap::new(),
		scopes: expected_scopes_btreemap,
	};
	let mut parsed_intermediate_representation = types::IntermediateRepresentation::default();
	input_ast.parse(
		types::ParserVariables::default(),
		&mut parsed_intermediate_representation,
	);
	//dbg!(&expected_intermediate_representation);
	//dbg!(&parsed_intermediate_representation);
	assert!(expected_intermediate_representation == parsed_intermediate_representation);
}
