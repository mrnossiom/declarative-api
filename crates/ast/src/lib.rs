//! Definitions of AST nodes and types

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

mod ptr;
mod types;

pub use ptr::P;
pub use types::*;