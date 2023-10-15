use crate::{SourceMap, Span};
use ariadne::{Report, ReportKind};
use core::fmt;
use parking_lot::Mutex;
#[cfg(debug_assertions)]
use std::panic::Location;
use std::rc::Rc;

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

	pub fn emit_diagnostic(&self, diag: &Diagnostic) {
		self.inner.lock().emit_diagnostic(diag);
	}

	#[track_caller]
	pub fn emit(&self, diag: impl Into<Diagnostic>) {
		self.emit_diagnostic(&diag.into());
	}

	pub fn print_statistics(&self) {
		let this = self.inner.lock();

		println!(
			"{} errors, {} warnings and {} advices were issued",
			this.error_count, this.warn_count, this.advice_count
		);
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
	fn emit_diagnostic(&mut self, diag: &Diagnostic) {
		match diag.report.kind {
			ReportKind::Error => self.error_count += 1,
			ReportKind::Warning => self.warn_count += 1,
			ReportKind::Advice => self.advice_count += 1,
			ReportKind::Custom(_, _) => {}
		};

		if let Err(err) = diag.report.eprint(self.source_map.to_cache_hack()) {
			tracing::error!("failed to print diagnostic: {}", err);
		};

		#[cfg(debug_assertions)]
		eprintln!("error was emitted here: {}", diag.loc);
	}
}

#[derive(Debug)]
pub struct Diagnostic {
	#[cfg(debug_assertions)]
	loc: &'static Location<'static>,
	report: Box<Report<'static, Span>>,
}

impl fmt::Display for Diagnostic {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.report.msg.clone().unwrap_or_default())
	}
}

impl Diagnostic {
	#[track_caller]
	#[must_use]
	pub fn new(report: Report<'static, Span>) -> Self {
		Self {
			#[cfg(debug_assertions)]
			loc: Location::caller(),
			report: Box::new(report),
		}
	}
}
