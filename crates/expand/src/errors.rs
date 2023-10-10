use macros::IntoDiagnostic;
use session::{Ident, Span};

#[derive(Debug, IntoDiagnostic)]
#[message("failed to load module `{name}`: {io_error}")]
pub(super) struct ExtScopeLoadingError {
	#[label("module `{name}` not found")]
	pub import: Span,
	pub name: Ident,
	pub io_error: std::io::Error,
}
