use super::*;
use hir_lowering::{utils, Parse};

#[test]
fn test_1() {
	let test_ir = utils::gen_test_ir();
	let mut generated_file_list = HashMap::new();
	test_ir.markdown(&mut generated_file_list);
	// generated_file_list.write(Path::new("test-artifacts/test-1"));
}

#[test]
#[allow(clippy::too_many_lines)]
fn test_2() {
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
			comment: Some("This is the API of Wiro".into()),
		},
		data: hir::types::ApiData {
			scopes: vec![hir::types::Scope {
				name: "dashboard".into(),
				models: vec![],
				scopes: vec![],
				methods: vec![],
				paths: vec![hir::types::Path {
					name: "dashboard".into(),
					scopes: vec![],
					paths: vec![],
					methods: vec![],
					headers: vec![
						hir::types::KeyValuePair {
							key: "Authorization".into(),
							type_: hir::types::Type::String,
							description: "The API Key of the User".into(),
							parameters: vec![("prefix".into(), "Api-Key".into())],
							comment: None,
						},
						hir::types::KeyValuePair {
							key: "X-Model".into(),
							type_: hir::types::Type::String,
							description: "The Model of the User".into(),
							parameters: vec![],
							comment: None,
						},
					],
					parameters: vec![],
					query: vec![],
					metadata: vec![],
					comment: None,
				}],
				headers: vec![],
				parameters: vec![],
				query: vec![],
				comment: None,
			}],
		},
	};
	let mut parsed_intermediate_representation = hir_lowering::types::Ir::default();
	input_hir.parse(
		hir_lowering::types::ParserVariables::default(),
		&mut parsed_intermediate_representation,
	);
	let mut generated_file_list = HashMap::new();
	parsed_intermediate_representation.markdown(&mut generated_file_list);
	// generated_file_list.write(Path::new("test-artifacts/test-2"));
}
