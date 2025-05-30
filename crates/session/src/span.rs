use crate::{
	SourceFile,
	source_map::{BytePos, FileIdx, with_source_map},
};
use std::{cmp, fmt};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Span {
	low: BytePos,
	high: BytePos,
}

impl Default for Span {
	fn default() -> Self {
		Self::DUMMY
	}
}

#[macro_export]
macro_rules! sp {
	($start:literal, $end:literal) => {
		$crate::Span::from_bounds($crate::BytePos($start), $crate::BytePos($end))
	};
}

impl Span {
	pub const DUMMY: Self = Self {
		low: BytePos(u32::MAX),
		high: BytePos(u32::MAX),
	};

	#[must_use]
	pub const fn from_bounds(low: BytePos, high: BytePos) -> Self {
		Self { low, high }
	}

	#[must_use]
	pub const fn low(&self) -> BytePos {
		self.low
	}

	#[must_use]
	pub const fn high(&self) -> BytePos {
		self.high
	}

	#[must_use]
	pub fn to(&self, span: Self) -> Self {
		Self {
			low: cmp::min(self.low, span.low),
			high: cmp::max(self.high, span.high),
		}
	}

	#[must_use]
	pub fn len(&self) -> BytePos {
		self.high - self.low
	}

	/// Returns the file index of the source file this span is in.
	///
	/// # Panics
	/// When used in a context where a source map is not available, this function will panic.
	#[must_use]
	#[track_caller]
	pub fn file_idx(&self) -> FileIdx {
		with_source_map(|sm| sm.lookup_source_file_index(self.low)).expect(
			"when retrieving the file index of a span, I need to be in a source map context",
		)
	}
}

impl fmt::Debug for Span {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&Self::DUMMY => write!(f, "Span(DUMMY)"),
			Self { low, high } => f
				.debug_struct("Span")
				.field("start", &low)
				.field("end", &high)
				.finish(),
		}
	}
}

impl fmt::Display for Span {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			&Self::DUMMY => write!(f, "a dummy span"),

			Self { low, high } => {
				let (start, end) = with_source_map(|sm| {
					let SourceFile { offset, .. } = *sm.lookup_source_file(*low);
					(*low - offset, *high - offset)
				})
				.unwrap_or((*low, *high));

				write!(f, "a span from {start} to {end}")
			}
		}
	}
}

impl ariadne::Span for Span {
	type SourceId = FileIdx;

	fn source(&self) -> &Self::SourceId {
		let idx = self.file_idx();

		// TODO: ugly, change this
		Box::leak(Box::new(idx))
	}

	fn start(&self) -> usize {
		self.low().to_usize()
	}

	fn end(&self) -> usize {
		self.high().to_usize()
	}
}
