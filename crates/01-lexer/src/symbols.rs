#![allow(non_upper_case_globals)]

use crate::span::Span;

macro_rules! symbols {
	{$group:ident, $($name:ident <== $match:literal),+} => {
		pub mod $group {
			$(
				pub const $name: super::Symbol = super::Symbol::new_static($match);
			)+
		}
	};
}

symbols! { kw,
	Model <== "model",
	Path <== "path",
	Meta <== "meta",
	Scope <== "scope"
}

symbols! { attrs,
	Doc <== "doc",
	Format <== "format"
}

// IDEA: change this to an index
// pub struct Symbol(u32);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Symbol(&'static str);

impl Symbol {
	#[must_use]
	pub const fn new_static(ident: &'static str) -> Self {
		Self(ident)
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
