use dapic_macros::IntoDiagnostic;
use dapic_session::Span;

#[derive(Debug, IntoDiagnostic)]
#[message("we found an invalid identifier")]
pub(crate) struct InvalidIdent {
	#[label("invalid identifier")]
	pub span: Span,
}

#[derive(Debug, IntoDiagnostic)]
#[message("OVLO (Unidentified lexeme object)")]
pub(crate) struct Unknown {
	#[label("🛸")]
	pub span: Span,
}
