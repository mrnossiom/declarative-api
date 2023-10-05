//! Declarative API Parser
//!
//! The main part is [`Parser`]. It takes a stream of rich lexer
//! [`Tokens`](lexer::rich::Token) and parses them into a tree of AST nodes.

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
	clippy::module_name_repetitions,
	clippy::missing_errors_doc
)]

mod error;
mod parser;

pub use crate::{error::PResult, parser::Parser};
