//! Declarative API data associated with a compilation session
//!
//! Among others: session context, diagnostics, source map, etc.

use crate::source_map::SOURCE_MAP;
use std::{
	cell::Cell,
	rc::Rc,
	time::{Duration, Instant},
};

mod diagnostics;
mod id;
mod macros;
mod source_map;
mod span;
#[path = "symbols.rs"]
mod symbols_;

pub use crate::{
	diagnostics::{Diagnostic, DiagnosticsHandler},
	id::{Idx, IndexVec},
	source_map::{BytePos, SourceFile, SourceFileHash, SourceFileId, SourceMap, with_source_map},
	span::Span,
	symbols_::{Ident, Symbol},
};

pub mod symbols {
	pub use crate::symbols_::{attrs, kw, remarkable};
}

/// This is there to avoid having to add `ariadne` in crates that uses `IntoDiagnostic` macro
#[doc(hidden)]
pub mod __private {
	pub use ariadne;
}

/// Represents the data associated with a compilation session.
#[derive(Debug)]
pub struct Session {
	pub diagnostics: DiagnosticsHandler,
	pub source_map: Rc<SourceMap>,
	timer: Timer,
}

impl Default for Session {
	fn default() -> Self {
		let source_map = Rc::<SourceMap>::default();

		Self {
			diagnostics: DiagnosticsHandler::new(source_map.clone()),
			source_map,
			timer: Timer::default(),
		}
	}
}

impl Session {
	/// Provide [`SourceMap`] context through [`with_source_map`]
	pub fn enter_source_map_ctx<T>(&mut self, f: impl FnOnce(&mut Self) -> T) -> T {
		SOURCE_MAP.with(|sm| *sm.borrow_mut() = Some(self.source_map.clone()));
		let value = f(self);
		SOURCE_MAP.with(|sm| sm.borrow_mut().take());
		value
	}

	pub const fn parse_sess(&self) -> ParseSession<'_> {
		ParseSession {
			diag: &self.diagnostics,
		}
	}

	pub fn time(&mut self, label: &'static str) -> TimerGuard {
		self.timer.now(label)
	}
}

/// Info about a parsing session.
#[derive(Debug)]
pub struct ParseSession<'a> {
	pub diag: &'a DiagnosticsHandler,
}

#[derive(Debug, Default)]
struct Timer {
	registered: Vec<(&'static str, Rc<Cell<Duration>>)>,
}

impl Timer {
	fn now(&mut self, label: &'static str) -> TimerGuard {
		let cell = Rc::<Cell<_>>::default();
		self.registered.push((label, cell.clone()));
		TimerGuard {
			start: Instant::now(),
			cell,
		}
	}

	fn print(&self) {
		if !self.registered.is_empty() {
			println!("--- Timers:");
			for (name, time) in &self.registered {
				eprintln!("{name}: {}μs", time.get().as_micros());
			}
			println!("---");
		}
	}
}

impl Drop for Timer {
	fn drop(&mut self) {
		self.print();
	}
}

pub struct TimerGuard {
	start: Instant,
	cell: Rc<Cell<Duration>>,
}

impl TimerGuard {
	pub fn run<T>(self, timed: impl FnOnce() -> T) -> T {
		let _timer = self;
		timed()
	}
}

impl Drop for TimerGuard {
	fn drop(&mut self) {
		self.cell.set(self.start.elapsed());
	}
}
