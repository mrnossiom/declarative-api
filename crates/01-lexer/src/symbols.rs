#![allow(non_upper_case_globals)]

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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol(&'static str);

impl Symbol {
	#[must_use]
	pub const fn new_static(ident: &'static str) -> Self {
		Self(ident)
	}
}
