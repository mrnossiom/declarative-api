use std::str::FromStr;

use crate::{types, utils, Parse};

#[test]
#[allow(clippy::too_many_lines)]
fn test_3() {
	use hir::types::*;

	let input_hir = Api {
		metadata: ApiMetadata {
			name: Some("Wiro's API".into()),
			version: Some(Version {
				major: 1,
				minor: 0,
				patch: 0,
			}),
			licence: None,
			urls: vec![
				"https://paradigm.lighton.ai/api/v1".into(),
				"https://paradigm-preprod.lighton.ai/api/v1".into(),
				"https://paradigm-dev.lighton.ai/api/v1".into(),
			],
			comment: Some("Je suis un commentaire de documentation (des métadonnées)".into()),
		},
		data: ApiData {
			scopes: vec![Scope {
				name: "dashboard".into(),
				scopes: vec![],
				models: vec![Model {
					name: "metrics".into(),
					model_body: vec![
						KeyValuePair {
							key: "name".into(),
							type_: Type::String,
							description: "The name of the metric".into(),
							parameters: vec![],
							comment: None,
						},
						KeyValuePair {
							key: "email".into(),
							type_: Type::String,
							description: String::new(),
							parameters: vec![],
							comment: None,
						},
						KeyValuePair {
							key: "password".into(),
							type_: Type::String,
							description: String::new(),
							parameters: vec![],
							comment: None,
						},
					],
					comment: None,
				}],
				paths: vec![Path {
					name: "dashboard".into(),
					scopes: vec![],
					paths: vec![Path {
						name: "metrics".into(),
						paths: vec![],
						comment: None,
						methods: vec![],
						headers: vec![KeyValuePair {
							key: "Authorization".into(),
							type_: Type::String,
							description: "The authorization key.".into(),
							parameters: vec![],
							comment: None,
						}],
						metadata: vec![],
						parameters: vec![],
						query: vec![],
						scopes: vec![Scope {
							name: "metrics".into(),
							scopes: vec![],
							models: vec![],
							paths: vec![],
							methods: vec![Method {
									method: http::Method::from_str("GET").expect(
												"a method used for testing the hir-lowering crate doesn't exist",
											),
											responses: vec![
												Response{
													headers: vec![],
													status_code: 200,
													body:vec![
                                                    KeyValuePair{
														key:"metrics".into(),
														type_:Type::Model("metrics".into()),
														description:String::new(),
														parameters:vec![],
														comment: None,
													},KeyValuePair{
														key:"status".into(),
														type_:Type::Object(
															vec![
																KeyValuePair{
																	key:"message".into(),
																	type_:Type::String,
																	description:"The status message itself".into(),
																	parameters:vec![],
																	comment: None,
																},KeyValuePair{
																	key:"code".into(),
																	type_:Type::Int,
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

	let mut parsed_ir = types::Ir::default();
	input_hir.parse(types::ParserVariables::default(), &mut parsed_ir);

	let expected_ir = utils::gen_test_ir();

	assert!(expected_ir == parsed_ir);
}
