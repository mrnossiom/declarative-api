pub(super) mod scope {
	use dapic_macros::IntoDiagnostic;
	use dapic_session::{Diagnostic, Ident, Span};

	#[derive(Debug, IntoDiagnostic)]
	#[message("found two candidate to import, but can only choose one between `{sibling_candidate}` and `{child_candidate}`")]
	pub struct MultipleCandidates {
		pub import_name: Ident,
		#[label("multiple candidates found for this scope `{import_name}`")]
		pub import: Span,

		pub sibling_candidate: String,
		pub child_candidate: String,
	}

	#[derive(Debug, IntoDiagnostic)]
	#[message("failed to find a valid candidate for `{import_name}` between `{sibling_candidate}` and `{child_candidate}`")]
	pub struct NoCandidate {
		pub import_name: Ident,
		#[label("no file candidates for `{import_name}`")]
		pub import: Span,

		pub sibling_candidate: String,
		pub child_candidate: String,
	}

	#[derive(Debug, IntoDiagnostic)]
	#[message("failed to load module `{import_name}`: {io}")]
	pub struct LoadingError {
		pub import_name: Ident,
		#[label("could not read candidate for this scope `{import_name}`")]
		pub import: Span,

		pub io: std::io::Error,
	}

	#[derive(Debug, IntoDiagnostic)]
	#[message("failed to parse module `{import_name}` : {parsing_err}")]
	pub struct ParsingError {
		pub import_name: Ident,
		#[label("could not parse candidate for `{import_name}`")]
		pub import: Span,
		pub parsing_err: Diagnostic,
	}

	#[derive(Debug, IntoDiagnostic)]
	#[message("module `{import_name}` was already imported creating a cyclic dependency. Stack: {import_stack}")]
	pub struct CyclicImport {
		pub import_name: Ident,
		#[label("module `{import_name}`")]
		pub import: Span,

		pub import_stack: String,
	}
}
