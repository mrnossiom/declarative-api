//! Data associated with a compilation session
//!
//! e.g. session context with diagnostics and source map

#![warn(
	// clippy::missing_docs_in_private_items,
	clippy::unwrap_used,
	clippy::nursery,
	clippy::pedantic,
	clippy::todo,
	clippy::dbg_macro,
	rustdoc::all,
)]
#![allow(
	clippy::redundant_pub_crate,
	clippy::enum_glob_use,
	clippy::module_name_repetitions
)]

mod diagnostics;
mod macros;
mod source_map;
mod span;
#[path = "symbols.rs"]
mod symbols_;

use std::{
	cell::Cell,
	collections::HashMap,
	rc::Rc,
	time::{Duration, Instant},
};

pub use diagnostics::{Diagnostic, DiagnosticsHandler};
// pub use macros::{ident, sp, sym};
pub use source_map::{
	add_source_map_context, with_source_map, BytePos, SourceFile, SourceFileHash, SourceFileId,
	SourceMap,
};
pub use span::Span;
pub use symbols_::{Ident, Symbol};

pub mod symbols {
	pub use crate::symbols_::{attrs, kw, remarkable};
}

/// This is there to avoid having to import `ariadne` in crate that uses IntoDiagnostic macro
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
	registered: HashMap<&'static str, Rc<Cell<Duration>>>,
}

impl Timer {
	fn now(&mut self, label: &'static str) -> TimerGuard {
		if self.registered.get(label).is_none() {
			let cell = Rc::<Cell<_>>::default();
			self.registered.insert(label, cell.clone());
			TimerGuard {
				start: Instant::now(),
				cell,
			}
		} else {
			panic!("timer label `{label}` already registered");
		}
	}

	fn print(&self) {
		println!("Timers:");
		println!("---");
		for (name, time) in &self.registered {
			eprintln!("{name}: {}μs", time.get().as_micros());
		}
		println!("---");
	}
}

// TODO: should be explicit
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
		let result = timed();
		drop(self);
		result
	}
}

impl Drop for TimerGuard {
	fn drop(&mut self) {
		self.cell.set(self.start.elapsed());
	}
}

// --- Macros ---
// Needs to be top level to be used in other crates

#[macro_export]
macro_rules! sym {
	($sym:literal) => {
		$crate::Symbol::intern($sym)
	};
}

#[macro_export]
macro_rules! ident {
	($name:literal, $start:literal, $end:literal) => {
		ident!($name, $crate::sp!($start, $end))
	};
	($name:literal, $span:expr) => {
		$crate::Ident::new($crate::Symbol::intern($name), $span)
	};
}

#[macro_export]
macro_rules! sp {
	($start:literal, $end:literal) => {
		$crate::Span::from_bounds($crate::BytePos($start), $crate::BytePos($end))
	};
}
