//! Macros to simplify some bits of code.
//!
//! These are mainly proc-macros and Derive macros.

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

use synstructure::decl_derive;

mod diagnostics;
mod symbols;

#[proc_macro]
pub fn symbols(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	symbols::symbols(input.into()).into()
}

decl_derive!(
	[IntoDiagnostic, attributes(/* block */ message, error, severity, /* fields */ label)] =>
	/// You can use the derive as such:
	/// ```rs
	/// #[derive(Debug, IntoDiagnostic)]
	/// #[message("we found an invalid ident {ident}")] // You can use template strings
	/// #[error("E0001")] // By default, we display the struct's name
	/// #[severity(Warning)] // By default, `Error` is used
	/// struct InvalidIdent {
	/// 	// You can use a template string here too
	/// 	//
	/// 	// You can insert `primary, ` before the literal when you
	/// 	// have multiple spans to force this one to be the first.
	/// 	#[label("invalid ident")]
	/// 	ident: Ident,
	/// }
	/// ```
	diagnostics::diagnostics
);
