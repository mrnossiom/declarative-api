use crate::P;

use super::{Expr, Path};
use lexer::rich::{Delimiter, Token};
use session::{Ident, Span, Symbol};
use std::{
	fmt,
	sync::atomic::{AtomicU32, Ordering},
};
use thin_vec::ThinVec;

pub type AttrVec = ThinVec<Attribute>;

/// An attribute of these forms
///
/// `@name`
/// `@name(<tokens>)`
/// `@name{<tokens>}`
/// `@name[<tokens>]`
///
/// or an outer doc comment
///
/// `## doc comment`
///
/// with an optional `!` after the `@` or the `##` to signify inner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute {
	pub kind: AttrKind,
	pub style: AttrStyle,

	pub id: AttrId,
	pub span: Span,
}

/// Define the style of the attribute, corresponding to the optional `!`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttrStyle {
	/// Without a `!` bang
	OuterOrInline,

	/// With a `!` bang
	Inner,
}

impl fmt::Display for AttrStyle {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Inner => write!(f, "inner"),
			Self::OuterOrInline => write!(f, "outer or inline"),
		}
	}
}

impl From<lexer::rich::AttrStyle> for AttrStyle {
	fn from(value: lexer::rich::AttrStyle) -> Self {
		match value {
			lexer::rich::AttrStyle::Inner => Self::Inner,
			lexer::rich::AttrStyle::OuterOrInline => Self::OuterOrInline,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttrKind {
	/// A normal attribute. Which keeps tokens to be later processed.
	Normal(NormalAttr),

	/// A simple key-value attribute (e.g. `@key: "value"` where "value" can be any expr).
	Meta(MetaAttr),

	/// A doc comment (e.g. `## ...`, `##! ...`).
	/// Doc attributes (e.g. `@doc("...")`) are represented with the `Normal`
	/// variant (which is much less compact and thus more expensive).
	DocComment(Symbol),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AttrId(u32);

impl AttrId {
	pub fn make_id() -> Self {
		static NEXT_ID: AtomicU32 = AtomicU32::new(0);

		Self(NEXT_ID.fetch_add(1, Ordering::Relaxed))
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalAttr {
	pub path: Path,
	pub delim: Delimiter,
	pub tokens: ThinVec<Token>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetaAttr {
	pub ident: Ident,
	pub expr: Option<P<Expr>>,
}
