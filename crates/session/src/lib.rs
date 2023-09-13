#![warn(
	// clippy::missing_docs_in_private_items,
	clippy::unwrap_used,
	clippy::nursery,
	clippy::pedantic,
	clippy::todo,
	clippy::dbg_macro,
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

use std::rc::Rc;

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

/// Represents the data associated with a compilation session.
#[derive(Debug, Default)]
pub struct Session {
	pub parse: ParseSession,
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
		ident!(
			$name,
			$crate::Span {
				start: $crate::BytePos($start),
				end: $crate::BytePos($end),
			}
		)
	};
	($name:literal, $span:expr) => {
		$crate::Ident::new($crate::Symbol::intern($name), $span)
	};
}

#[macro_export]
macro_rules! sp {
	($start:literal, $end:literal) => {
		$crate::Span {
			start: $crate::BytePos($start),
			end: $crate::BytePos($end),
		}
	};
}
