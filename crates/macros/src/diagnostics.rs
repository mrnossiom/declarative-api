use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
	parse::Parse, punctuated::Punctuated, Attribute, Expr, ExprLit, Fields, FieldsNamed,
	FieldsUnnamed, Lit, LitStr, Meta, MetaNameValue, Token,
};
use synstructure::Structure;

#[allow(clippy::too_many_lines)]
pub(crate) fn diagnostics(mut s: Structure) -> syn::Result<TokenStream> {
	let fields = match &s.ast().data {
		syn::Data::Struct(struct_) => struct_.fields.clone(),
		syn::Data::Enum(_enum_) => unimplemented!("doesn't derive enums"),
		syn::Data::Union(_union_) => unimplemented!("doesn't derive unions"),
	};

	let mut error_code: Option<LitStr> = None;
	let mut severity: Option<Ident> = None;
	let mut message: Option<LitStr> = None;
	let mut labels: Vec<(Ident, String)> = vec![];

	for meta in extract_diag_attrs(&s.ast().attrs) {
		if meta.path().is_ident("code") {
			// Parse `code("<...>")`
			let str_ = meta
				.require_list()
				.expect("a single str literal enclosed in parenthesis")
				.parse_args_with(<LitStr as Parse>::parse)?;

			error_code = Some(str_);
		} else if meta.path().is_ident("severity") {
			// Parse `severity(Error)`, `severity(Advice)`, etc.
			let ident = meta
				.require_list()
				.expect("a single str literal enclosed in parenthesis")
				.parse_args_with(<Ident as Parse>::parse)?;

			// TODO: support custom severities e.g. `severity("some custom String")`

			severity = Some(ident);
		} else if meta.path().is_ident("msg") {
			// Parse `msg("<...>")`
			let str_ = meta
				.require_list()
				.expect("a single str literal enclosed in parenthesis")
				.parse_args_with(<LitStr as Parse>::parse)?;

			message = Some(str_);
		} else {
			return Err(syn::Error::new_spanned(
				meta,
				"this attribute is not supported on the struct",
			));
		}
	}

	for field in &fields {
		let ident = field.ident.as_ref().expect("field to have name");

		for meta in extract_diag_attrs(&field.attrs) {
			if meta.path().is_ident("label") {
				// Parse `label = "expected {style}"`
				let expr = match meta {
					Meta::Path(_) => None,
					Meta::NameValue(MetaNameValue { ref value, .. }) => Some(value),
					Meta::List(_) => return Err(syn::Error::new_spanned(&meta, "label can have a display string like so `label = \"expected {style}\"` but not a list")),
				};

				match &field.ty {
					syn::Type::Path(path) if path.path.is_ident("Span") => {}
					_ => {
						return Err(syn::Error::new_spanned(
							&field.ty,
							"a label can only be of type `session::Span`",
						))
					}
				}

				let label_desc = match expr {
					Some(Expr::Lit(ExprLit {
						lit: Lit::Str(lit), ..
					})) => lit.value(),
					None => return Err(syn::Error::new_spanned(&meta, "label can have a display string like so `label = \"expected {style}\"` but not a path")),
					_ => return Err(syn::Error::new_spanned(&meta, "label can have a display string like so `label = \"expected {style}\"` but not another kind of expression")),
				};

				labels.push((ident.clone(), label_desc));
			} else {
				return Err(syn::Error::new_spanned(
					meta,
					"this attribute is not supported on struct fields",
				));
			}
		}
	}

	let error_code =
		error_code.unwrap_or_else(|| LitStr::new(&s.ast().ident.to_string(), Span::call_site()));
	let severity = severity.unwrap_or_else(|| format_ident!("Error"));
	let message =
		message.ok_or_else(|| syn::Error::new_spanned(s.ast(), "expected a `msg` attribute"))?;

	let pat = fields_pat(&fields);

	let color_bindings = fields
		.iter()
		.map(|field| {
			let ident = field.ident.as_ref().expect("field to have name");
			let renamed = renamed(ident);

			quote!(let #ident = #renamed.to_string().fg(__color.next());)
		})
		.collect::<TokenStream>();

	let Some(main_span) = labels.first().map(|(ident, _)| renamed(ident)) else {
		// TODO: add a way to specify the main span

		return Err(syn::Error::new_spanned(
			s.ast(),
			"expected at least one label",
		));
	};

	let quoted_labels = labels.iter().map(|(ident, message)| {
		let renamed = renamed(ident);

		quote!(Label::new(#renamed).with_message(format!(#message)))
	});

	let color_state: [u16; 3] = [0, 0, 0].map(|_| fastrand::u16(..));

	s.underscore_const(true);
	Ok(s.gen_impl(quote! {
		use ::ariadne::{ColorGenerator, Config, Fmt, Label, Report, ReportKind, LabelAttach};
		use ::session::Diagnostic;

		gen impl Into<Diagnostic> for @Self {
			fn into(self) -> Diagnostic {
				let Self #pat = self;

				let mut __color = ColorGenerator::from_state([#(#color_state),*], 0.5);
				#color_bindings

				// TODO
				Report::build(ReportKind::#severity, #main_span.file_idx(), #main_span.offset().as_usize())
					.with_code(#error_code)
					.with_message(format!(#message))
					#(.with_label(#quoted_labels))*
					.with_config(Config::default().with_label_attach(LabelAttach::Middle))
					.finish()
					.into()
			}
		}
	}))
}

fn fields_pat(fields: &Fields) -> TokenStream {
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

	quote!({ #(#fields),* })
}

fn extract_diag_attrs(attrs: &[Attribute]) -> Vec<Meta> {
	attrs
		.iter()
		.filter_map(|a| {
			if a.meta.path().is_ident("diag") {
				Some(&a.meta)
			} else {
				None
			}
		})
		.map(|a| a.require_list().expect("require a list"))
		.map(|a| {
			a.parse_args_with(Punctuated::<syn::Meta, Token![,]>::parse_terminated)
				.expect("a meta")
		})
		.fold(Vec::default(), |mut vec, ml| {
			vec.extend(ml);
			vec
		})
}

fn renamed(ident: &Ident) -> Ident {
	format_ident!("__original_{}", ident)
}
