#![warn(
	// clippy::missing_docs_in_private_items,
	clippy::unwrap_used,
	clippy::nursery,
	clippy::pedantic,
)]
#![allow(
	clippy::redundant_pub_crate,
	clippy::enum_glob_use,
	clippy::module_name_repetitions,
	clippy::missing_errors_doc
)]

mod error;
mod parser;

pub use crate::{
	error::{PError, PResult},
	parser::Parser,
};
