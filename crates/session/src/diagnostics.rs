use crate::{SourceMap, Span};
use ariadne::{Report, ReportKind};
use core::fmt;
use parking_lot::Mutex;
#[cfg(debug_assertions)]
use std::panic::Location;
use std::{process, rc::Rc};

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

	pub fn emit_fatal_diagnostic(&self, diag: &Diagnostic) -> ! {
		self.inner.lock().emit_diagnostic(diag);
		// TODO: print fatal warning
		process::exit(1)
	}

	#[track_caller]
	pub fn emit(&self, diag: impl Into<Diagnostic>) {
		self.emit_diagnostic(&diag.into());
	}

	/// Prints diagnostics statistics and exits if multiple errors were reported.
	#[allow(clippy::significant_drop_tightening)]
	pub fn check_degraded_and_exit(&self) {
		let this = self.inner.lock();

		if this.degraded() {
			println!(
				"{} errors, {} warnings and {} advices were issued",
				this.error_count, this.warn_count, this.advice_count
			);

			process::exit(1);
		}
	}

	/// Prints stats on the number of non-fatal errors issued.
	///
	/// # Panics
	/// Must not be called in a degraded state
	pub fn print_final_stats(&self) {
		let this = self.inner.lock();

		assert!(
			!this.degraded(),
			"check degraded before printing final stats"
		);

		if this.warn_count != 0 || this.advice_count != 0 {
			println!(
				"{} warnings and {} advices were issued",
				this.warn_count, this.advice_count
			);
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

	const fn degraded(&self) -> bool {
		self.error_count != 0
	}
}

#[derive(Debug)]
pub struct Diagnostic {
	report: Box<Report<'static, Span>>,
	#[cfg(debug_assertions)]
	loc: &'static Location<'static>,
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
			report: Box::new(report),
			#[cfg(debug_assertions)]
			loc: Location::caller(),
		}
	}
}
