
use std::{
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
	let input_hir = hir::types::Api {
		metadata: hir::types::ApiMetadata {
			name: Some("Wiro's API".into()),
			version: Some(hir::types::Version {
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
		data: hir::types::ApiData {
			child_scopes: vec![hir::types::Scope {
				name: "dashboard".into(),
				child_scopes: vec![],
				child_models: vec![hir::types::Model{
					name:"metrics".into(),
					model_body: vec![hir::types::KeyValuePair{
						key:"name".into(),
						type_:hir::types::Type::String,
						description:"The name of the metric".into(),
						parameters:vec![],
						comment:None,
					},hir::types::KeyValuePair{
						key:"email".into(),
						type_:hir::types::Type::String,
						description:String::new(),
						parameters:vec![],
						comment:None,
					},hir::types::KeyValuePair{
						key:"password".into(),
						type_:hir::types::Type::String,
						description:String::new(),
						parameters:vec![],
						comment:None,
					},
					],
				comment:None,
				}],
				child_paths: vec![hir::types::Path {
					name: "dashboard".into(),
					child_scopes: vec![],
					child_paths: vec![hir::types::Path {
						name: "metrics".into(),
						child_paths: vec![],
						comment: None,
						methods: vec![],
						headers: vec![hir::types::KeyValuePair {
							key: "Authorization".into(),
							type_: hir::types::Type::String,
							description: "The authorization key.".into(),
							parameters: vec![],
							comment: None,
						}],
						metadata: vec![],
						parameters: vec![],
						query: vec![],
						child_scopes: vec![hir::types::Scope {
							name: "metrics".into(),
							child_scopes: vec![],
							child_models: vec![],
							child_paths: vec![],
							methods: vec![hir::types::Method {
									method: http::Method::from_str("GET").expect(
												"a method used for testing the hir-lowering crate doesn't exist",
											),
											responses: vec![
												hir::types::Response{
													headers: vec![],
													status_code: 200,
													body:vec![
                                                        
                                                    hir::types::KeyValuePair{
														key:"metrics".into(),
														type_:hir::types::Type::Model("metrics".into()),
														description:String::new(),
														parameters:vec![],
														comment: None,
													},hir::types::KeyValuePair{
														key:"status".into(),
														type_:hir::types::Type::Object(
															vec![
																hir::types::KeyValuePair{
																	key:"message".into(),
																	type_:hir::types::Type::String,
																	description:"The status message itself".into(),
																	parameters:vec![],
																	comment: None,
																},hir::types::KeyValuePair{
																	key:"code".into(),
																	type_:hir::types::Type::Int,
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
	
	
	let mut parsed_intermediate_representation = types::IntermediateRepresentation::default();
	input_hir.parse(
		types::ParserVariables::default(),
		&mut parsed_intermediate_representation,
	);
	let expected_intermediate_representation = utils::gen_test_ir();
	//dbg!(&expected_intermediate_representation);
	//dbg!(&parsed_intermediate_representation);
	assert!(expected_intermediate_representation == parsed_intermediate_representation);
}
