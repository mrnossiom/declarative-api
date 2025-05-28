//! Proc macro which builds the base Symbol table

use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::{cell::RefCell, collections::HashMap};
use syn::{
	Ident, LitStr, Token, braced,
	parse::{Parse, ParseStream, Result},
};

enum SymbolGroupElement {
	Symbol(Symbol),
	/// Corresponds to [`syn::token::Minus`](struct@syn::token::Minus)
	Minus,
}

struct Symbol {
	name: Ident,
	value: Option<LitStr>,
}

impl Parse for Symbol {
	fn parse(input: ParseStream<'_>) -> Result<Self> {
		let name = input.parse()?;
		let value = match input.parse::<Token![:]>() {
			Ok(_) => Some(input.parse()?),
			Err(_) => None,
		};

		Ok(Self { name, value })
	}
}

struct Input {
	static_ident: Ident,
	groups: Vec<(Ident, Vec<SymbolGroupElement>)>,
}

impl Default for Input {
	fn default() -> Self {
		Self {
			static_ident: Ident::new("FRESH_SYMBOLS", Span::call_site()),
			groups: vec![],
		}
	}
}

impl Parse for Input {
	fn parse(input: ParseStream<'_>) -> Result<Self> {
		let static_ident = input.parse()?;
		input.parse::<Token![,]>()?;

		let mut groups = vec![];

		while let Ok(ident) = input.parse::<Ident>() {
			let content;
			braced!(content in input);

			let mut symbols = vec![];

			loop {
				if content.is_empty() {
					break;
				}

				if content.parse::<Token![-]>().is_ok() {
					// This is used to break ordering of symbols.
					symbols.push(SymbolGroupElement::Minus);
				}

				// First parse a symbol `Sym: "pat"`
				symbols.push(SymbolGroupElement::Symbol(Symbol::parse(&content)?));

				if content.is_empty() {
					break;
				}

				let _punctuation: Token![,] = content.parse()?;
			}

			groups.push((ident, symbols));
		}

		Ok(Self {
			static_ident,
			groups,
		})
	}
}

#[derive(Default)]
struct Errors {
	list: Vec<syn::Error>,
}

impl Errors {
	fn error(&mut self, span: Span, message: String) {
		self.list.push(syn::Error::new(span, message));
	}
}

pub fn symbols(input: TokenStream) -> TokenStream {
	let (mut output, errors) = symbols_with_errors(input);

	// If we generated any errors, then report them as compiler_error!() macro calls.
	// This lets the errors point back to the most relevant span. It also allows us
	// to report as many errors as we can during a single run.
	output.extend(errors.into_iter().map(|e| e.to_compile_error()));

	output
}

fn symbols_with_errors(input: TokenStream) -> (TokenStream, Vec<syn::Error>) {
	let mut errors = Errors::default();

	let Input {
		groups,
		static_ident,
	} = syn::parse2(input).unwrap_or_else(|e| {
		// This allows us to display errors at the proper span, while minimizing
		// unrelated errors caused by bailing out (and not generating code).
		errors.list.push(e);

		Input::default()
	});

	let mut symbols_stream = quote! {};
	let mut prefill_stream = quote! {};
	let mut counter = 0u32;

	let mut keys = HashMap::<String, Span>::default();

	let mut check_dup = |span: Span, str_: &str, errors: &mut Errors| {
		#[allow(clippy::nursery)]
		if let Some(prev_span) = keys.get(str_) {
			errors.error(span, format!("Symbol `{str_}` is duplicated"));
			errors.error(*prev_span, "location of previous definition".to_string());
		} else {
			keys.insert(str_.to_string(), span);
		}
	};

	for (ident, symbols) in groups {
		let prev_key: RefCell<Option<(Span, String)>> = RefCell::default();

		let check_order = |span: Span, str: &str, errors: &mut Errors| {
			if let Some((prev_span, ref prev_str)) = *prev_key.borrow_mut()
				&& str < prev_str
			{
				errors.error(span, format!("Symbol `{str}` must precede `{prev_str}`"));
				errors.error(
					prev_span,
					format!("location of previous symbol `{prev_str}`"),
				);
			}

			*prev_key.borrow_mut() = Some((span, str.to_string()));
		};

		let mut current_symbols_stream = quote! {};

		// Generate the listed symbols.
		for symbol in &symbols {
			let symbol = match symbol {
				SymbolGroupElement::Symbol(symbol) => symbol,
				SymbolGroupElement::Minus => {
					*prev_key.borrow_mut() = None;
					continue;
				}
			};

			let name = &symbol.name;
			let value = symbol
				.value
				.as_ref()
				.map_or_else(|| name.to_string(), LitStr::value);

			check_dup(symbol.name.span(), &value, &mut errors);
			check_order(symbol.name.span(), &name.to_string(), &mut errors);

			current_symbols_stream.extend(quote! {
				pub const #name: Symbol = Symbol::new(#counter);
			});
			prefill_stream.extend(quote! { #value, });

			counter += 1;
		}

		symbols_stream.extend(quote! {
			#[allow(non_upper_case_globals)]
			pub mod #ident {
				use super::Symbol;
				#current_symbols_stream
			}
		});
	}

	let output = quote! {
		static #static_ident: &[&str] = &[#prefill_stream];
		#symbols_stream
	};

	(output, errors.list)
}
