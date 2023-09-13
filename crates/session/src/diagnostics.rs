use std::fmt;
use std::{error::Error, rc::Rc};

use crate::SourceMap;
use crate::Span;
use ariadne::{Report, ReportKind};
use parking_lot::Mutex;

#[derive(Debug, Default)]
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
		self.inner.lock().emit_diagnostic(diag);
	}

	pub fn emit(&self, diag: impl Into<Diagnostic>) {
		self.emit_diagnostic(diag.into());
	}
}

#[derive(Debug, Default)]
struct InnerHandler {
	source_map: Rc<SourceMap>,

	error_count: u32,
	warn_count: u32,
	advice_count: u32,
}

impl InnerHandler {
	fn emit_diagnostic(&mut self, diag: Diagnostic) {
		match diag.0.kind {
			ReportKind::Error => self.error_count += 1,
			ReportKind::Warning => self.warn_count += 1,
			ReportKind::Advice => self.advice_count += 1,
			ReportKind::Custom(_, _) => {}
		};

		// TODO: check if span is valid

		// TODO: this doesn't work with labels in different files, or with more than one file (which is even more annoying)

		diag.0.eprint(self.source_map.to_cache_hack()).unwrap();
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
pub struct Diagnostic(Report<'static, Span>);

impl fmt::Display for Diagnostic {
	fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
		todo!()
	}
}

impl Error for Diagnostic {
	fn cause(&self) -> Option<&dyn Error> {
		None
	}
}

impl From<Report<'static, Span>> for Diagnostic {
	fn from(value: Report<'static, Span>) -> Self {
		Self(value)
	}
}
