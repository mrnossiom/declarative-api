use crate::P;
use lexer::{
	span::Span,
	symbols::{kw, Symbol},
};

mod attr;
mod expr;
mod item;
mod node;
pub use attr::*;
pub use expr::*;
pub use item::*;
pub use node::*;
use thin_vec::ThinVec;

#[derive(Debug, Clone)]
pub struct Api {
	pub attrs: AttrVec,
	pub items: ThinVec<P<Item>>,

	pub id: NodeId,
	pub span: Span,
}

// TODO: nope, not a String
#[derive(Debug, Clone)]
pub struct Type(pub(crate) String);

#[derive(Debug, Clone)]
pub struct Ident {
	pub symbol: Symbol,
	pub span: Span,
}

impl Ident {
	pub const EMPTY: Self = Self {
		symbol: kw::Empty,
		span: Span::DUMMY,
	};

	#[must_use]
	pub const fn new(symbol: Symbol, span: Span) -> Self {
		Self { symbol, span }
	}
}

impl From<lexer::symbols::Ident> for Ident {
	fn from(lexer::symbols::Ident { span, symbol }: lexer::symbols::Ident) -> Self {
		Self { symbol, span }
	}
}
