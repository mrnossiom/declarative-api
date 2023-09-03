use std::rc::Rc;

mod diagnostics;
mod source_map;
mod span;
#[path = "symbols.rs"]
mod symbols_;

pub use diagnostics::{Diagnostic, DiagnosticSource, DiagnosticsHandler, IntoDiagnostic};
pub use source_map::{BytePos, SourceFile, SourceFileHash, SourceFileId, SourceMap};
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
