use crate::{
	source_map::{with_source_map, BytePos, FileIdx},
	SourceFile,
};
use std::{cmp, fmt};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Span {
	pub start: BytePos,
	pub end: BytePos,
}

impl Span {
	pub const DUMMY: Self = Self {
		start: BytePos(u32::MAX),
		end: BytePos(u32::MAX),
	};

	#[must_use]
	pub const fn from_bounds(start: BytePos, end: BytePos) -> Self {
		Self { start, end }
	}

	#[must_use]
	pub fn to(&self, span: Self) -> Self {
		Self {
			start: cmp::min(self.start, span.start),
			end: cmp::max(self.end, span.end),
		}
	}

	#[must_use]
	pub fn len(&self) -> BytePos {
		self.end - self.start
	}

	#[must_use]
	pub const fn offset(&self) -> BytePos {
		// TODO: check is valid

		self.start
	}

	#[must_use]
	pub fn file_idx(&self) -> FileIdx {
		with_source_map(|sm| sm.lookup_source_file_index(self.start))
			.expect("to be in a source map context")
	}

	#[must_use]
	pub fn relative_to(&self, file: &SourceFile) -> Self {
		Self {
			start: self.start - file.start_pos,
			end: self.end - file.start_pos,
		}
	}
}

impl fmt::Debug for Span {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&Self::DUMMY => write!(f, "Span(DUMMY)"),
			Self { start, end } => f
				.debug_struct("Span")
				.field("start", &start)
				.field("end", &end)
				.finish(),
		}
	}
}

impl fmt::Display for Span {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			&Self::DUMMY => write!(f, "DUMMY"),
			Self { start, end } => write!(f, "{start} -> {end}"),
		}
	}
}

impl ariadne::Span for Span {
	type SourceId = FileIdx;

	fn source(&self) -> &Self::SourceId {
		let idx = with_source_map(|sm| sm.lookup_source_file_index(self.start).clone()).unwrap();

		let idx = Box::leak(Box::new(idx));

		idx
	}

	fn start(&self) -> usize {
		let idx = self.source();

		let start_pos = with_source_map(|sm| sm.files.read().sources[idx].start_pos).unwrap();

		(self.start - start_pos).as_usize()
	}

	fn end(&self) -> usize {
		let idx = self.source();

		let start_pos = with_source_map(|sm| sm.files.read().sources[idx].start_pos).unwrap();

		(self.end - start_pos).as_usize()
	}
}