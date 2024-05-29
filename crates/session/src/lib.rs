//! Declarative API data associated with a compilation session
//!
//! Among others: session context, diagnostics, source map, etc.

mod diagnostics;
mod id;
mod macros;
mod source_map;
mod span;
#[path = "symbols.rs"]
mod symbols_;

use std::{
	cell::Cell,
	rc::Rc,
	time::{Duration, Instant},
};

pub use diagnostics::{Diagnostic, DiagnosticsHandler};
pub use id::{Idx, IndexVec};
pub use source_map::{
	add_source_map_context, with_source_map, BytePos, SourceFile, SourceFileHash, SourceFileId,
	SourceMap,
};
pub use span::Span;
pub use symbols_::{Ident, Symbol};

pub mod symbols {
	pub use crate::symbols_::{attrs, kw, remarkable};
}

/// This is there to avoid having to add `ariadne` in crates that uses `IntoDiagnostic` macro
#[doc(hidden)]
pub mod __private {
	pub use ariadne;
}

/// Represents the data associated with a compilation session.
#[derive(Debug, Default)]
pub struct Session {
	pub parse: ParseSession,
	timer: Timer,
}

impl Session {
	pub fn time(&mut self, label: &'static str) -> TimerGuard {
		self.timer.now(label)
	}
}

impl Drop for Session {
	fn drop(&mut self) {
		self.timer.print();
		self.parse.diagnostic.print_statistics();
	}
}

/// Info about a parsing session.
#[derive(Debug)]
pub struct ParseSession {
	pub diagnostic: DiagnosticsHandler,
	pub source_map: Rc<SourceMap>,
}

impl Default for ParseSession {
	fn default() -> Self {
		let source_map = Rc::<SourceMap>::default();

		Self {
			diagnostic: DiagnosticsHandler::new(source_map.clone()),
			source_map,
		}
	}
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
