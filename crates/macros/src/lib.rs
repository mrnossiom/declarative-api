//! Declarative API compiler-specific macros
//!
//! These are often created to reduce boilerplate code.

use synstructure::decl_derive;

mod diagnostics;
mod symbols;

/// Generates modules of `Symbol`s and a static array to initialize an interner.
///
/// # Example
///
/// This will generate modules `tag` and `tag2` and preintern the symbols.
///
/// ```ignore
/// symbols! { FRESH_SYMBOLS,
///     tag {
///         Sym: "pattern",
///         Sym2: "pattern2",
///
///         - // Separate parts of the group with a dash
///         Sym3: "pattern3",
///         Sym4: "pattern4",
///     }
///
///     tag2 {
///         Sym5: "pattern5",
///     }
/// }
/// ```
#[proc_macro]
pub fn symbols(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	symbols::symbols(input.into()).into()
}

decl_derive!(
	[IntoDiagnostic, attributes(/* block */ message, error, severity, /* fields */ label)] =>
	/// Implements `dapic_session::Diagnostic` trait on error structs with convinience.
	///
	/// You can use the derive as such:
	///
	/// ```ignore
	/// #[derive(Debug, IntoDiagnostic)]
	/// #[message("we found an invalid ident {ident}")] // You can use template strings
	/// #[error("E0001")] // By default, we display the struct's name
	/// #[severity(Warning)] // By default, `Error` is used
	/// struct InvalidIdent {
	/// 	// You can use a template string here too.
	/// 	//
	/// 	// You can insert `primary, ` before the literal when you
	/// 	// have multiple spans to force this one to be the first.
	/// 	#[label("invalid ident")]
	/// 	ident: Ident,
	/// }
	/// ```
	diagnostics::diagnostics
);
