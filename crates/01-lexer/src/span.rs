use std::{cmp, fmt};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Span {
	pub start: u32,
	pub end: u32,
}

impl Span {
	pub const DUMMY: Self = Self { start: 0, end: 0 };

	#[must_use]
	pub const fn from_bounds(lo: u32, hi: u32) -> Self {
		Self { start: lo, end: hi }
	}

	#[must_use]
	pub fn to(&self, span: Self) -> Self {
		Self {
			start: cmp::min(self.start, span.start),
			end: cmp::max(self.end, span.end),
		}
	}
}

// Recursive expansion of Debug macro
// ===================================

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
