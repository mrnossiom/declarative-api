use std::fmt;

/// Identifies an AST node.
///
/// This identifies top-level definitions, expressions, and everything in between.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(u32);

impl NodeId {
	/// The [`NodeId`] used to represent the root of the crate.
	pub const ROOT: Self = Self(0);

	/// When parsing and at the beginning of doing expansions, we initially give all AST nodes
	/// this dummy AST [`NodeId`]. Then, during a later phase of expansion, we renumber them
	/// to have small, positive IDs.
	pub const DUMMY: Self = Self(u32::MAX);
}

impl fmt::Display for NodeId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Display::fmt(&self.0, f)
	}
}
