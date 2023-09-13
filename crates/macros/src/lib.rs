#![warn(
	// clippy::missing_docs_in_private_items,
	clippy::unwrap_used,
	clippy::nursery,
	clippy::pedantic,
	clippy::todo,
	clippy::dbg_macro,
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

decl_derive!([IntoDiagnostic, attributes(diag)] => diagnostics::diagnostics);
