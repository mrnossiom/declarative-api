use fastrand::Rng;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use std::{collections::hash_map::DefaultHasher, hash::Hasher};
use syn::{
	parse::{Parse, ParseStream},
	parse_quote, Attribute, Field, Fields, FieldsNamed, FieldsUnnamed, Ident, LitStr, Meta, Token,
	Type,
};
use synstructure::Structure;

mod attrs {
	pub(super) const MESSAGE: &str = "message";
	pub(super) const LABEL: &str = "label";
	pub(super) const ERROR_CODE: &str = "code";
	pub(super) const SEVERITY: &str = "severity";

	pub(super) const ALL: [&str; 4] = [MESSAGE, LABEL, ERROR_CODE, SEVERITY];
}

macro_rules! bail {
	($tokens:expr, $msg:literal) => {
		return Err(syn::Error::new_spanned($tokens, $msg))
	};
}

#[allow(clippy::too_many_lines)]
pub(crate) fn diagnostics(mut s: Structure) -> syn::Result<TokenStream> {
	let fields = match &s.ast().data {
		syn::Data::Struct(struct_) => struct_.fields.clone(),
		syn::Data::Enum(enum_) => bail!(
			enum_.enum_token,
			"enums can't be derived with `IntoDiagnostic`"
		),
		syn::Data::Union(union_) => bail!(
			union_.union_token,
			"unions can't be derived with `IntoDiagnostic`"
		),
	};

	// --- Struct Meta ---

	let mut error_code: Option<String> = None;
	let mut severity: Option<Ident> = None;
	let mut message: Option<String> = None;

	for meta in extract_diag_attrs(&s.ast().attrs)
		.map(StructMeta::new)
		.collect::<syn::Result<Vec<_>>>()?
	{
		match meta {
			StructMeta::Code(meta, code) => {
				if error_code.replace(code).is_some() {
					bail!(
						meta,
						"this attribute replaces value of the previous `code` attribute"
					)
				}
			}

			StructMeta::Message(meta, msg) => {
				if message.replace(msg).is_some() {
					bail!(
						meta,
						"this attribute replaces value of the previous `message` attribute"
					)
				}
			}

			StructMeta::Severity(meta, s) => {
				if severity.replace(s).is_some() {
					bail!(
						meta,
						"this attribute replaces value of the previous `message` attribute"
					)
				}
			}
		}
	}

	let error_code = error_code.unwrap_or_else(|| s.ast().ident.to_string());
	let severity = severity.unwrap_or_else(|| format_ident!("Error"));
	let message = message
		.ok_or_else(|| syn::Error::new_spanned(&s.ast().ident, "expected a `message` attribute"))?;

	// --- Fields Meta ---

	let mut labels: Vec<Label> = vec![];
	let mut main_span: Option<Label> = None;

	for field in &fields {
		for meta in extract_diag_attrs(&field.attrs)
			.map(|meta| FieldMeta::new(field.clone(), meta))
			.collect::<syn::Result<Vec<_>>>()?
		{
			match meta {
				FieldMeta::Label(label) => {
					match &field.ty {
						// We don't need no restriction on `Span`s since we use convert
						// to this type and assume it implements Display
						syn::Type::Path(path) if path.path.is_ident("Span") => {}
						_ => {
							let ty = &field.ty;
							// Restrictions
							let display_restriction = quote!(::core::fmt::Display);
							let as_ref_restriction = quote!(AsRef<Span>);
							let into_restriction = quote!(Into<Span>);

							s.add_where_predicate(parse_quote!(#ty: #display_restriction));
							s.add_where_predicate(parse_quote!(#ty: #as_ref_restriction));
							s.add_where_predicate(parse_quote!(#ty: #into_restriction));
						}
					}

					if label.primary && main_span.replace(label.clone()).is_some() {
						bail!(label, "this label is defined as primary but there is already another primary label")
					}

					labels.push(label);
				}
			}
		}
	}

	let main_span = match main_span.as_ref().or_else(|| labels.first()) {
		Some(sp) => {
			let ident = renamed(&sp.ident);

			match &sp.ty {
				Type::Path(path) if path.path.is_ident("Span") => quote!(#ident),
				_ => quote!(#ident.as_ref()),
			}
		}
		None => bail!(&fields, "expected at least one label"),
	};

	// Unpack every field
	let fields_unpacked = unpack_fields_renamed(&fields);

	let mut struct_name_derived_rng = {
		let mut hash = DefaultHasher::default();
		hash.write(s.ast().ident.to_string().as_bytes());
		Rng::with_seed(hash.finish())
	};

	let color_bindings = fields
		.iter()
		.map(|field| {
			let ident = field.ident.as_ref().expect("field to have name");
			let renamed = renamed(ident);

			// We remove colors that aren't interesting (black, white, etc. and their shades)
			// See <https://www.ditig.com/256-colors-cheat-sheet>
			let color_id = struct_name_derived_rng.u8(9..231);
			quote!(let #ident = #renamed.to_string().fg(Color::Fixed(#color_id));)
		})
		.collect::<TokenStream>();

	s.underscore_const(true);
	Ok(s.gen_impl(quote! {
		use ::dapic_session::__private::ariadne::{Color, Config, Fmt, Label, Report, ReportKind, LabelAttach};
		use ::dapic_session::{with_source_map, Diagnostic, Span};

		#[automatically_derived]
		gen impl Into<Diagnostic> for @Self {
			#[track_caller]
			fn into(self) -> Diagnostic {
				#fields_unpacked
				#color_bindings

				let report = Report::build(ReportKind::#severity, #main_span.file_idx(), #main_span.low().to_char_pos().to_usize())
					.with_code(#error_code)
					.with_message(format!(#message))
					#(.with_label(#labels))*
					.with_config(Config::default().with_label_attach(LabelAttach::Middle))
					.finish();

				Diagnostic::new(report)
			}
		}
	}))
}

/// Produces unpacking like `let Self { field1: __original_field1, field2: __original_field2 } = self;`
fn unpack_fields_renamed(fields: &Fields) -> TokenStream {
	let fields = match fields {
		Fields::Named(FieldsNamed { named, .. }) => named,
		Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => unnamed,
		Fields::Unit => return quote!({}),
	}
	.iter()
	.map(|field| field.ident.clone().expect("field has a name"))
	.map(|ident| {
		let renamed = renamed(&ident);
		quote!(#ident: #renamed)
	});

	quote!(let Self { #(#fields),* } = self;)
}

/// We need to rename attributes to keep their original value while derive users
/// can use template strings that renders random colored Displays implementations
fn renamed(ident: &Ident) -> Ident {
	format_ident!("__original_{}", ident)
}

/// Filters attributes with the ones we are concerned
fn extract_diag_attrs(attrs: &[Attribute]) -> impl Iterator<Item = Meta> + '_ {
	attrs
		.iter()
		.filter(|attr| {
			attr.meta
				.path()
				.get_ident()
				.map_or(false, |ident| attrs::ALL.contains(&&*ident.to_string()))
		})
		.map(|attr| attr.meta.clone())
}

enum StructMeta {
	Severity(Meta, Ident),
	Message(Meta, String),
	Code(Meta, String),
}

impl StructMeta {
	fn new(meta: Meta) -> syn::Result<Self> {
		let kind = if meta.path().is_ident(attrs::ERROR_CODE) {
			// Parse `code("<...>")`

			let str_ = meta
				.require_list()
				.expect("a single str literal enclosed in parenthesis")
				.parse_args_with(<LitStr as Parse>::parse)?;

			Self::Code(meta, str_.value())
		} else if meta.path().is_ident(attrs::SEVERITY) {
			// Parse `severity(Error)`, `severity(Advice)`, etc.
			let ident = meta
				.require_list()
				.expect("a single str literal enclosed in parenthesis")
				.parse_args_with(<Ident as Parse>::parse)?;

			// TODO: check if it is a valid severity
			// IDEA: support custom severities e.g. `severity("some custom String")`

			Self::Severity(meta, ident)
		} else if meta.path().is_ident(attrs::MESSAGE) {
			// Parse `message("<...>")`
			let str_ = meta
				.require_list()
				.map_err(|_| {
					syn::Error::new_spanned(&meta, "the message attribute must a single string literal enclosed in parenthesis, e.g. msg(\"explication\")")
				})?
				.parse_args_with(<LitStr as Parse>::parse)?;

			Self::Message(meta, str_.value())
		} else {
			bail!(
				meta,
				"this attribute is not supported by the `IntoDiagnostic` derive"
			)
		};

		Ok(kind)
	}
}

enum FieldMeta {
	Label(Label),
}

impl FieldMeta {
	fn new(field: Field, meta: Meta) -> syn::Result<Self> {
		let kind = if meta.path().is_ident(attrs::LABEL) {
			Self::Label(Label::new(field, &meta)?)
		} else {
			bail!(meta, "this attribute is not supported on struct fields");
		};

		Ok(kind)
	}
}

#[derive(Debug, Clone)]
struct Label {
	ident: Ident,
	message: String,

	ty: Type,
	primary: bool,
}

impl Label {
	/// Parses `label(main, "expected {style}")` or `label("expected {style}")`
	fn new(field: Field, meta: &Meta) -> syn::Result<Self> {
		struct LabelInner {
			tag: Option<Ident>,
			message: String,
		}

		impl Parse for LabelInner {
			fn parse(input: ParseStream) -> syn::Result<Self> {
				let ident = if input.peek(Ident) {
					let ident: Ident = input.parse()?;
					input.parse::<Token![,]>()?;

					Some(ident)
				} else {
					None
				};

				let message = input.parse::<LitStr>()?.value();

				if !input.is_empty() {
					bail!(ident, "")
				}

				Ok(Self {
					tag: ident,
					message,
				})
			}
		}

		let path = meta.require_list()?;

		let LabelInner { tag, message } = syn::parse2(path.tokens.clone())?;

		let primary = if let Some(ident) = tag {
			if ident == "primary" {
				true
			} else {
				bail!(ident, "this tag is not supported")
			}
		} else {
			false
		};

		Ok(Self {
			ident: field
				.clone()
				.ident
				.ok_or_else(|| syn::Error::new_spanned(&field, "expected a named field"))?,
			message,
			ty: field.ty,
			primary,
		})
	}
}

impl ToTokens for Label {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let Self { ident, message, .. } = self;

		let renamed = renamed(ident);
		tokens.extend(quote!(Label::new(#renamed.into()).with_message(format!(#message))));
	}
}
