use crate::span::Span;
use macros::symbols;
use parking_lot::Mutex;
use std::fmt;
use std::mem;
use std::{collections::HashMap, fmt::Display};
use typed_arena::Arena;

thread_local! {
	static SYMBOL_INTERNER: SymbolInterner = SymbolInterner::fresh();
}

symbols! {
	kw {

		Auth: "auth",
		Body: "body",
		// should be at the top, but we have to fix ordering first
		Empty: "",
		Enum: "enum",
		False: "false",
		Headers: "headers",
		Meta: "meta",
		Model: "model",
		Params: "params",
		Path: "path",
		Query: "query",
		Scope: "scope",
		True: "true",
	}

	attrs {
		Deprecated: "deprecated",
		Doc: "doc",
		Format: "format",
		Type: "type",
	}

	remarkable {
		Delete: "DELETE",
		Get: "GET",
		Post: "POST",
		Put: "PUT",
	}
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Symbol(u32);

impl fmt::Debug for Symbol {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			&kw::Empty => write!(f, "Symbol(Empty)"),
			Self(id) => write!(f, r#"Symbol({} "{}")"#, id, self.as_str()),
		}
	}
}

impl Symbol {
	#[must_use]
	const fn new(id: u32) -> Self {
		Self(id)
	}

	#[must_use]
	pub fn intern(sym: &str) -> Self {
		SYMBOL_INTERNER.with(|interner| interner.intern(sym))
	}

	/// Access the underlying string. This is a slowish operation because it
	/// requires locking the symbol interner.
	///
	/// Note that the lifetime of the return value is a lie. It's not the same
	/// as `&self`, but actually tied to the lifetime of the underlying
	/// interner. Interners are long-lived, and there are very few of them, and
	/// this function is typically used for short-lived things, so in practice
	/// it works out ok.
	#[must_use]
	pub fn as_str(&self) -> &str {
		SYMBOL_INTERNER
			.with(|interner| unsafe { mem::transmute::<&str, &str>(interner.get(*self)) })
	}
}

impl Display for Symbol {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.as_str())
	}
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ident {
	pub symbol: Symbol,
	pub span: Span,
}

impl Ident {
	#[must_use]
	pub const fn new(symbol: Symbol, span: Span) -> Self {
		Self { symbol, span }
	}
}

struct SymbolInterner(Mutex<InnerSymbolInterner>);

struct InnerSymbolInterner {
	arena: Arena<u8>,
	names: HashMap<&'static str, Symbol>,
	strings: Vec<&'static str>,
}

impl SymbolInterner {
	fn prefill(init: &[&'static str]) -> Self {
		Self(Mutex::new(InnerSymbolInterner {
			arena: Arena::default(),
			strings: init.into(),
			names: init.iter().copied().zip((0..).map(Symbol::new)).collect(),
		}))
	}

	#[tracing::instrument(level = "DEBUG", skip(self))]
	fn intern(&self, sym: &str) -> Symbol {
		let mut this = self.0.lock();
		if let Some(&name) = this.names.get(sym) {
			return name;
		}

		let name = Symbol::new(this.strings.len() as u32);

		let id = this.arena.alloc_str(sym);

		// TODO: check safety
		let id = unsafe { &*(id as *mut str as *const str) };

		this.names.insert(id, name);
		this.strings.push(id);

		name
	}

	// Get the symbol as a string. `Symbol::as_str()` should be used in
	// preference to this function.
	fn get(&self, symbol: Symbol) -> &str {
		self.0.lock().strings[symbol.0 as usize]
	}
}
