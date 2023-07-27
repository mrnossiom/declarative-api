use std::collections::{HashMap, HashSet};

use super::{types, BTreeMap};

#[must_use]
#[allow(clippy::too_many_lines)]
pub fn gen_test_ir() -> types::IntermediateRepresentation {
	let mut expected_endpoint_object = types::Endpoint::default();
	expected_endpoint_object.methods.insert(
		http::Method::GET.to_string(),
		types::Method {
			responses: BTreeMap::from([(
				200,
				types::Response {
					body: BTreeMap::from([
						(
							"metrics".into(),
							types::KeyValuePairValue {
								type_: types::Type::ResolvedModel(vec![
									"dashboard".into(),
									"metrics".into(),
								]),
								description: String::new(),
								parameters: BTreeMap::new(),
								comment: None,
							},
						),
						(
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
						),
					]),
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
				scopes: HashSet::from(["dashboard".into()]),
				models: BTreeMap::new(),
				endpoints: HashMap::new(),
				comment: None,
			},
		),
		// Level 1 scope
		(
			vec!["dashboard".into()],
			types::Scope {
				scopes: HashSet::from(["metrics".into()]),
				models: BTreeMap::from([(
					"metrics".into(),
					types::Model {
						model_body: BTreeMap::from([
							(
								"name".into(),
								types::KeyValuePairValue {
									type_: types::Type::String,
									description: "The name of the metric".into(),
									parameters: BTreeMap::new(),
									comment: None,
								},
							),
							(
								"email".into(),
								types::KeyValuePairValue {
									type_: types::Type::String,
									description: String::new(),
									parameters: BTreeMap::new(),
									comment: None,
								},
							),
							(
								"password".into(),
								types::KeyValuePairValue {
									type_: types::Type::String,
									description: String::new(),
									parameters: BTreeMap::new(),
									comment: None,
								},
							),
						]),
						comment: None,
					},
				)]),
				endpoints: HashMap::new(),
				comment: None,
			},
		),
		// Level 2 scope
		(
			vec!["dashboard".into(), "metrics".into()],
			types::Scope {
				scopes: HashSet::new(),
				models: BTreeMap::new(),
				endpoints: HashMap::from([("/dashboard/metrics".into(), expected_endpoint_object)]),
				comment: None,
			},
		),
	]);
	types::IntermediateRepresentation {
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
		scopes: expected_scopes_btreemap,
	}
}
