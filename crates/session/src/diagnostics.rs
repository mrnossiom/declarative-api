use crate::{SourceFile, SourceMap};
use miette::Severity;
use parking_lot::Mutex;
use std::{error::Error, fmt::Display, rc::Rc};

pub trait DiagnosticSource: miette::Diagnostic {
	fn source_file(&self, source_map: &SourceMap) -> Rc<SourceFile>;
}

#[derive(Debug)]
pub struct DiagnosticsHandler {
	inner: Mutex<InnerHandler>,
}

impl DiagnosticsHandler {
	pub fn new(source_map: Rc<SourceMap>) -> Self {
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
		self.inner.lock().emit_diagnostic(diag)
	}

	pub fn emit(&self, diag: impl miette::Diagnostic + DiagnosticSource + Send + Sync + 'static) {
		let diag = self.builder(diag);

		self.emit_diagnostic(diag);
	}

	#[must_use]
	pub fn builder(
		&self,
		diag: impl miette::Diagnostic + DiagnosticSource + Send + Sync + 'static,
	) -> Diagnostic {
		let sm = &self.inner.lock().source_map;
		let sf = diag.source_file(sm);

		Diagnostic {
			report: miette::Report::new(diag).with_source_code((*sf.source).clone()),
		}
	}
}

#[derive(Debug)]
struct InnerHandler {
	source_map: Rc<SourceMap>,

	error_count: u32,
	warn_count: u32,
	advice_count: u32,
}

impl InnerHandler {
	fn emit_diagnostic(&mut self, diag: Diagnostic) {
		match diag.report.severity().unwrap_or_default() {
			Severity::Error => self.error_count += 1,
			Severity::Warning => self.warn_count += 1,
			Severity::Advice => self.advice_count += 1,
		};

		println!("{:?}", diag.report)
	}
}

// TODO: maybe log manually, not on a drop?
impl Drop for InnerHandler {
	fn drop(&mut self) {
		println!(
			"{} errors, {} warnings and {} advices were issued",
			self.error_count, self.warn_count, self.advice_count
		)
	}
}

#[derive(Debug)]
pub struct Diagnostic {
	report: miette::Report,
}

impl Display for Diagnostic {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.report.fmt(f)
	}
}

impl Error for Diagnostic {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		self.report.source()
	}
}

pub trait IntoDiagnostic<'a> {
	fn into_diag(self, handler: &'a DiagnosticsHandler) -> Diagnostic;
}
