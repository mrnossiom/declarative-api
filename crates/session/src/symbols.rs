use crate::span::Span;
use macros::symbols;
use parking_lot::Mutex;
use std::{
	collections::HashMap,
	fmt::{self, Display},
	mem,
};
use tracing::instrument;
use typed_arena::Arena;

thread_local! {
	static SYMBOL_INTERNER: SymbolInterner = SymbolInterner::prefill(FRESH_SYMBOLS);
}

symbols! { FRESH_SYMBOLS,
	kw {
		- // Non-constructible symbols that are used as markers
		Empty: "",
		PathRoot: "{{root}}",

		- // Keywords
		Auth: "auth",
		Body: "body",
		Code: "code",
		Enum: "enum",
		Headers: "headers",
		Meta: "meta",
		Model: "model",
		Params: "params",
		Path: "path",
		Query: "query",
		Scope: "scope",
		Verb: "verb",

		- // Bool literals
		False: "false",
		True: "true"
	}

	attrs {
		deprecated,
		description,
		doc,
		format,
		r#type: "type",
	}

	remarkable {
		- // HTTP methods
		Connect: "CONNECT",
		Delete: "DELETE",
		Get: "GET",
		Head: "HEAD",
		Options: "OPTIONS",
		Post: "POST",
		Put: "PUT",
		Trace: "TRACE",
	}
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Symbol(u32);

impl fmt::Debug for Symbol {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			&kw::Empty => write!(f, "Symbol(Empty)"),
			Self(id) => write!(f, r#"Symbol({}, "{}")"#, id, self.as_str()),
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
		SYMBOL_INTERNER.with(|interner| unsafe {
			// SAFETY: Interner is long-lived whereas symbols references are dropped when file-gen has been completed
			mem::transmute::<&str, &str>(interner.get(*self))
		})
	}
}

impl Display for Symbol {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.as_str())
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl fmt::Display for Ident {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if f.alternate() {
			write!(f, "an ident {}", self.symbol)
		} else {
			self.symbol.fmt(f)
		}
	}
}

impl AsRef<Span> for Ident {
	fn as_ref(&self) -> &Span {
		&self.span
	}
}

impl From<Ident> for Span {
	fn from(val: Ident) -> Self {
		val.span
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

	#[instrument(level = "DEBUG", skip(self))]
	fn intern(&self, sym: &str) -> Symbol {
		let mut this = self.0.lock();
		if let Some(&name) = this.names.get(sym) {
			return name;
		}

		let id = u32::try_from(this.strings.len())
			.expect("Wow, you've just inserted 2^32 symbols into 4 GiB of source code");
		let name = Symbol::new(id);

		let id = this.arena.alloc_str(sym);

		// TODO: check safety
		// SAFETY: ?
		let id = unsafe { &*(id as *mut str).cast_const() };

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
