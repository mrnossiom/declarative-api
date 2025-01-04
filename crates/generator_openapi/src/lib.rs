use dapic_hir::types as hir;
use indexmap::IndexMap;
use openapiv3::{
	Components, Info, IntegerType, OpenAPI, Paths, ReferenceOr, Schema, SchemaData, SchemaKind,
	Type,
};

// Public exports
pub use serde_json;

fn models_to_schemas(models: &[&hir::Item]) -> IndexMap<String, ReferenceOr<Schema>> {
	let mut map = IndexMap::new();

	map.insert(
		String::from("hello"),
		ReferenceOr::Item(Schema {
			schema_kind: SchemaKind::Type(Type::Integer(IntegerType::default())),
			schema_data: SchemaData::default(),
		}),
	);

	// for model in models {}

	map
}

#[must_use]
pub fn generate_openapi_spec(crate_: &hir::Root) -> OpenAPI {
	// let meta = Option::<hir::Meta>::None;
	let mut models = Vec::<&hir::Item>::new();

	for item in crate_.items() {
		match item.kind {
			hir::ItemKind::Model => models.push(item),
		}
	}

	let schemas = models_to_schemas(&models);

	let components = Components {
		schemas,
		..Default::default()
	};

	OpenAPI {
		openapi: "3".into(),
		info: Info {
			title: "my_spec".into(),
			version: "0.0.0".into(),
			..Default::default()
		},
		servers: vec![],
		paths: Paths::default(),
		components: Some(components),
		security: None,
		tags: vec![],
		external_docs: None,
		..Default::default()
	}
}
