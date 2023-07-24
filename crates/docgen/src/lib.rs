mod types;

pub trait DocOutput {
	fn generate(meta: ast::types::Api) -> types::Documentation;
}
