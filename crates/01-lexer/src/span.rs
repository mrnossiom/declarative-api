#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
	pub start: u32,
	pub end: u32,
}

impl Span {
	#[must_use]
	pub const fn from_bounds(lo: u32, hi: u32) -> Self {
		Self { start: lo, end: hi }
	}

	#[must_use]
	pub const fn dummy() -> Self {
		Self { start: 0, end: 0 }
	}
}
