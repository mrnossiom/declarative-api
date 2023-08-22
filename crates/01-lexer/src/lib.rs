#![warn(
	// clippy::missing_docs_in_private_items,
	clippy::unwrap_used,
	clippy::nursery,
	clippy::pedantic,
)]
#![allow(
	clippy::redundant_pub_crate,
	clippy::enum_glob_use,
	clippy::module_name_repetitions
)]

pub mod poor;
pub mod rich;
pub mod span;
pub mod symbols;

#[cfg(test)]
mod tests {
	pub const EXAMPLE: &str = include_str!("../../../examples/wiro-api/main.dapi");

	pub const ATTR: &str = "@format: date";

	pub const URLS: &str = r#"urls [
	"https://paradigm.lighton.ai/api/v1"
	"https://paradigm-preprod.lighton.ai/api/v1"
	"https://paradigm-dev.lighton.ai/api/v1"
]"#;
}
