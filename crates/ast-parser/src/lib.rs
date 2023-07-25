#![allow(unused)]
use ast::types::Api;
use types::IntermediateRepresentation;

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
		parser_variables: types::ParserVariables,
		intermediate_representation: &mut types::IntermediateRepresentation,
	) {
		intermediate_representation.metadata.name = Some(self.name.clone().unwrap()); // Panic if this field is not set.
		let version = self.version.as_ref().unwrap(); // Panic if this field is not set.
		intermediate_representation.metadata.version = Some(types::Version {
			major: version.major,
			minor: version.minor,
			patch: version.patch,
		});
		intermediate_representation.metadata.urls = self.urls.clone();
		intermediate_representation.metadata.comment = self.comment.clone();
	}
}

impl Parse for ast::types::Api {
	fn parse(
		&self,
		parser_variables: types::ParserVariables,
		intermediate_representation: &mut types::IntermediateRepresentation,
	) {
		self.metadata
			.parse(parser_variables, intermediate_representation)
	}
}

#[cfg(test)]
mod tests {
	use std::collections::HashMap;

	use ast::types::Version;

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
				child_scopes: vec![],
			},
		};
		let expected_intermediate_representation = types::IntermediateRepresentation {
			metadata: types::ApiMetadata {
				name: Some("Wiro's API".into()),
				version: Some(types::Version {
					major: 1,
					minor: 0,
					patch: 0,
				}),
				urls: vec![],
				comment: Some("Je suis un commentaire de documentation (des métadonnées)".into()),
			},
			scopes: HashMap::new(),
		};
		let mut parsed_intermediate_representation = types::IntermediateRepresentation::new();
		input_ast.parse(
			types::ParserVariables::new(),
			&mut parsed_intermediate_representation,
		);
		assert!(expected_intermediate_representation == parsed_intermediate_representation);
	}
}
