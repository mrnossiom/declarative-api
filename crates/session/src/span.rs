use miette::SourceSpan;

use crate::{source_map::BytePos, SourceFile};
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

impl From<&SourceSpan> for Span {
	fn from(value: &SourceSpan) -> Self {
		Self {
			start: BytePos(value.offset() as u32),
			end: BytePos((value.offset() as u32) + (value.len() as u32)),
		}
	}
}

impl From<Span> for SourceSpan {
	fn from(sp: Span) -> Self {
		(sp.start.0 as usize, (sp.end - sp.start).0 as usize).into()
	}
}
