use crate::span::Span;
use std::fmt::Display;

macro_rules! symbols {
	{$group:ident, $($name:ident <== $match:literal),+,} => {
		#[allow(non_upper_case_globals)]
		pub mod $group {
			$(pub const $name: super::Symbol = super::Symbol::new_static($match);)+
		}
	};
}

symbols! { kw,
	Meta <== "meta",

	Scope <== "scope",
	Path <== "path",
	Auth <== "auth",

	Model <== "model",
	Enum <== "enum",

	Body <== "body",
	Headers <== "headers",
	Query <== "query",
	Params <== "params",
}

symbols! { attrs,
	Doc <== "doc",
	Format <== "format",
	Type <== "type",
	Deprecated <== "deprecated",
}

symbols! { remark,
	Get <== "GET",
	Post <== "POST",
	Put <== "PUT",
	Delete <== "DELETE",
}

// IDEA: change this to an index
// pub struct Symbol(u32);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Symbol(&'static str /* Should be an index type */);

impl Symbol {
	#[must_use]
	pub const fn new_static(ident: &'static str) -> Self {
		Self(ident)
	}
}

impl Display for Symbol {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ident {
	pub name: Symbol,
	pub span: Span,
}

impl Ident {
	#[must_use]
	pub const fn new(name: Symbol, span: Span) -> Self {
		Self { name, span }
	}
}
