use crate::SourceMap;
use miette::Severity;
use parking_lot::Mutex;
use std::{error::Error, fmt::Display, ops::Deref, sync::Arc};

#[derive(Debug)]
pub struct DiagnosticsHandler {
	inner: Mutex<InnerHandler>,
}

impl DiagnosticsHandler {
	pub fn new(source_map: Arc<SourceMap>) -> Self {
		let inner = InnerHandler {
			source_map,
			error_count: 0,
			warn_count: 0,
			advice_count: 0,
		};

		Self {
			inner: Mutex::new(inner),
		}
	}

	pub fn emit_diagnostic(&self, diag: Diagnostic) {
		self.inner.lock().emit_diagnostic(diag);
	}

	pub fn emit<'a>(&self, diag: impl IntoDiagnostic<'a>) {
		self.emit_diagnostic(diag.into_diag());
	}

	// TODO: change to `pub(crate)`
	#[must_use]
	pub fn builder(diag: impl miette::Diagnostic + Send + Sync + 'static) -> Diagnostic {
		// TODO: use named source to display file name
		Diagnostic(miette::Report::new(diag))
	}
}

#[derive(Debug)]
struct InnerHandler {
	source_map: Arc<SourceMap>,

	error_count: u32,
	warn_count: u32,
	advice_count: u32,
}

impl InnerHandler {
	fn emit_diagnostic(&mut self, diag: Diagnostic) {
		match diag.severity().unwrap_or_default() {
			Severity::Error => self.error_count += 1,
			Severity::Warning => self.warn_count += 1,
			Severity::Advice => self.advice_count += 1,
		};

		// TODO: check if span is valid

		// TODO: this doesn't work with labels in different files, or with more than one file (which is even more annoying)

		println!("{:?}", diag.0.with_source_code(self.source_map.clone()));
	}
}

// TODO: maybe log manually, not on a drop?
impl Drop for InnerHandler {
	fn drop(&mut self) {
		println!(
			"{} errors, {} warnings and {} advices were issued",
			self.error_count, self.warn_count, self.advice_count
		);
	}
}

#[derive(Debug)]
pub struct Diagnostic(miette::Report);

impl Deref for Diagnostic {
	type Target = miette::Report;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl Display for Diagnostic {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)
	}
}

impl Error for Diagnostic {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		self.0.source()
	}
}

pub trait IntoDiagnostic<'a> {
	fn into_diag(self) -> Diagnostic;
}
