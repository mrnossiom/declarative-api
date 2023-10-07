//! Declarative API Lexer
//!
//! Directly parses input sources in a stream of tokens for the parser.

#![warn(
	// clippy::missing_docs_in_private_items,
	clippy::unwrap_used,
	clippy::nursery,
	clippy::pedantic,
	clippy::todo,
	clippy::dbg_macro,
	rustdoc::all,
)]
#![allow(
	clippy::redundant_pub_crate,
	clippy::enum_glob_use,
	clippy::module_name_repetitions
)]

pub mod poor;
pub mod rich;

#[cfg(test)]
mod tests {
	pub const EXAMPLE: &str = include_str!("../../../examples/paradigm/paradigm.dapi");

	pub const ATTR: &str = "@format: date";

	pub const URLS: &str = r#"urls [
	"https://paradigm.lighton.ai/api/v1"
	"https://paradigm-preprod.lighton.ai/api/v1"
	"https://paradigm-dev.lighton.ai/api/v1"
]"#;
}
