use crate::{error::InvalidVerb, PResult, Parser};
use dapic_ast::types::{
	Api, AttrVec, Auth, Body, Enum, Headers, Item, ItemKind, Metadata, Model, NodeId, Params,
	PathItem, PathKind, Query, ScopeKind, StatusCode, Verb, P,
};
use dapic_lexer::rich::{Delimiter, OpKind, TokenKind};
use dapic_session::{
	symbols::{kw, remarkable},
	Ident,
};
use thin_vec::{thin_vec, ThinVec};

impl<'a> Parser<'a> {
	#[tracing::instrument(level = "DEBUG", skip(self))]
	pub fn parse_root(&mut self) -> PResult<Api> {
		let lo = self.token.span;

		let attrs = self.parse_inner_attrs()?;

		let mut items = ThinVec::default();
		items.push(self.parse_metadata()?);
		items.extend(self.parse_scope_content(None)?);

		Ok(Api {
			attrs,
			items,
			id: NodeId::ROOT,
			span: self.span(lo),
		})
	}

	#[tracing::instrument(level = "DEBUG", skip(self, attrs))]
	fn parse_scope(&mut self, attrs: &mut AttrVec) -> PResult<(Ident, ScopeKind)> {
		self.expect_keyword(kw::Scope)?;

		let ident = self.parse_ident()?;

		let scope = if self.eat(&TokenKind::Semi) {
			ScopeKind::Unloaded
		} else {
			let lo = self.token.span;
			let items = self.expect_braced(|p| p.parse_scope_content(Some(attrs)))?;

			ScopeKind::Loaded {
				items,
				inline: true,
				span: self.span(lo),
			}
		};

		Ok((ident, scope))
	}

	#[tracing::instrument(level = "DEBUG", skip(self, attrs))]
	pub fn parse_scope_content(
		&mut self,
		attrs: Option<&mut AttrVec>,
	) -> PResult<ThinVec<P<Item>>> {
		if let Some(attrs) = attrs {
			attrs.extend(self.parse_inner_attrs()?);
		}

		self.parse_items()
	}

	#[tracing::instrument(level = "DEBUG", skip(self))]
	fn parse_metadata(&mut self) -> PResult<P<Item>> {
		let lo = self.token.span;

		if !self.eat_keyword(kw::Meta) {
			todo!("react accordingly")
		}

		let fields = self.expect_braced(Self::parse_property_defs)?;

		let meta = Metadata { fields };

		Ok(Self::make_item(
			ThinVec::default(),
			ItemKind::Meta(meta),
			None,
			self.span(lo),
		))
	}

	#[tracing::instrument(level = "DEBUG", skip(self))]
	fn parse_items(&mut self) -> PResult<ThinVec<P<Item>>> {
		let mut items = ThinVec::default();
		while let Some(item) = self.parse_item()? {
			items.push(item);
		}
		Ok(items)
	}

	#[tracing::instrument(level = "DEBUG", skip(self))]
	fn parse_item(&mut self) -> PResult<Option<P<Item>>> {
		let mut attrs = self.parse_outer_attrs()?;

		let lo = self.token.span;

		// TODO: check keywords with case insensitive

		let (ident, kind) = if self.check_keyword(kw::Scope) {
			// `scope <ident> { <items> }`
			let (ident, kind) = self.parse_scope(&mut attrs)?;
			(Some(ident), ItemKind::Scope(kind))
		} else if self.check_keyword(kw::Path) {
			// `path <path> { <items> }`
			let path = self.parse_path_item()?;
			(None, ItemKind::Path(path))
		} else if self.check_keyword(kw::Meta) {
			// `meta { <properties> }`
			// TODO: change or emit warn about misplaced metadata, should go in passes
			let metadata = self.parse_metadata()?.kind.clone();
			(None, metadata)
		} else if self.check_keyword(kw::Headers) {
			// `headers { <fields> }`
			let headers = self.parse_headers()?;
			(None, ItemKind::Headers(headers))
		} else if self.check_keyword(kw::Query) {
			// `query { <fields> }`
			let query = self.parse_query()?;
			(None, ItemKind::Query(query))
		} else if self.check_keyword(kw::Code) {
			// `code <code lit> { <items> }`
			let item = self.parse_code_item()?;
			(None, ItemKind::StatusCode(item))
		} else if self.check_keyword(kw::Model) {
			// `model <ident> { <def_fields> }`
			let (ident, item) = self.parse_model()?;
			(Some(ident), ItemKind::Model(item))
		} else if self.check_keyword(kw::Enum) {
			// `enum <ident> { <def_fields> }`
			let (ident, item) = self.parse_enum()?;
			(Some(ident), ItemKind::Enum(item))
		} else if self.check_keyword(kw::Auth) {
			// `auth <ident> { <auth_fields> }`
			let (ident, item) = self.parse_auth()?;
			(Some(ident), ItemKind::Auth(item))
		} else if self.check_keyword(kw::Verb) {
			// `verb <http_verb_ident> { <items> }`
			let (ident, verb) = self.parse_verb()?;
			(Some(ident), ItemKind::Verb(verb))
		} else if self.check_keyword(kw::Body) {
			// `body <ty>`
			let body = self.parse_body()?;
			(None, ItemKind::Body(body))
		} else if self.check_keyword(kw::Params) {
			// `params { <field_defs> }`
			let params = self.parse_params()?;
			(None, ItemKind::Params(params))
		} else {
			if attrs.is_empty() {
				return Ok(None);
			}

			todo!("error if attributes are parsed but there is not item to attach them")
		};

		Ok(Some(Self::make_item(attrs, kind, ident, self.span(lo))))
	}

	#[tracing::instrument(level = "DEBUG", skip(self))]
	fn parse_path_item(&mut self) -> PResult<PathItem> {
		self.expect_keyword(kw::Path)?;
		let kind = self.parse_path_item_kind()?;
		let items = self.expect_braced(Self::parse_items)?;
		Ok(PathItem { kind, items })
	}

	#[tracing::instrument(level = "DEBUG", skip(self))]
	fn parse_path_item_kind(&mut self) -> PResult<PathKind> {
		let kind = if self.eat(&TokenKind::OpenDelim(Delimiter::Brace)) {
			let ident = self.parse_ident()?;
			self.expect(&TokenKind::CloseDelim(Delimiter::Brace))?;
			PathKind::Variable(ident)
		} else if self.eat(&TokenKind::Dot) {
			PathKind::Current
		} else {
			let ident = self.parse_ident()?;

			PathKind::Simple(ident)
		};

		if self.eat(&TokenKind::Op(OpKind::Slash)) {
			let mut parts = thin_vec![kind];

			match self.parse_path_item_kind()? {
				PathKind::Complex(vec) => parts.extend(vec),
				pk @ (PathKind::Simple(..) | PathKind::Variable(..) | PathKind::Current) => {
					parts.push(pk);
				}
			}

			Ok(PathKind::Complex(parts))
		} else {
			Ok(kind)
		}
	}

	#[tracing::instrument(level = "DEBUG", skip(self))]
	fn parse_headers(&mut self) -> PResult<Headers> {
		self.expect_keyword(kw::Headers)?;
		let headers = self.expect_braced(Self::parse_field_defs)?;
		Ok(Headers { headers })
	}

	#[tracing::instrument(level = "DEBUG", skip(self))]
	fn parse_query(&mut self) -> PResult<Query> {
		self.expect_keyword(kw::Query)?;
		let fields = self.expect_braced(Self::parse_field_defs)?;
		Ok(Query { fields })
	}

	#[tracing::instrument(level = "DEBUG", skip(self))]
	fn parse_code_item(&mut self) -> PResult<StatusCode> {
		self.expect_keyword(kw::Code)?;
		let code = self.parse_expr()?;
		let items = self.expect_braced(Self::parse_items)?;
		Ok(StatusCode { code, items })
	}

	#[tracing::instrument(level = "DEBUG", skip(self))]
	fn parse_verb(&mut self) -> PResult<(Ident, Verb)> {
		self.expect_keyword(kw::Verb)?;
		let method = self.parse_ident()?;
		let items = self.expect_braced(Self::parse_items)?;

		{
			// TODO: this is an experimental lint
			// this should be in a lint passes
			// also custom verbs are allowed

			use remarkable::{Connect, Delete, Get, Head, Options, Post, Put, Trace};
			if ![Connect, Delete, Get, Head, Options, Post, Put, Trace].contains(&method.symbol) {
				return Err(InvalidVerb { found: method }.into());
			}
		}

		Ok((method, Verb { method, items }))
	}

	#[tracing::instrument(level = "DEBUG", skip(self))]
	fn parse_model(&mut self) -> PResult<(Ident, Model)> {
		self.expect_keyword(kw::Model)?;
		let name = self.parse_ident()?;
		let fields = self.expect_braced(Self::parse_field_defs)?;
		Ok((name, Model { fields }))
	}

	#[tracing::instrument(level = "DEBUG", skip(self))]
	fn parse_enum(&mut self) -> PResult<(Ident, Enum)> {
		self.expect_keyword(kw::Enum)?;
		let name = self.parse_ident()?;
		let variants = self.expect_braced(Self::parse_property_defs)?;
		Ok((name, Enum { variants }))
	}

	#[tracing::instrument(level = "DEBUG", skip(self))]
	fn parse_auth(&mut self) -> PResult<(Ident, Auth)> {
		self.expect_keyword(kw::Auth)?;
		let auth_name = self.parse_ident()?;

		let kind = if self.eat(&TokenKind::Semi) {
			// `auth BasicAuth;`
			Auth::Use
		} else {
			// `auth BasicAuth { <field_defs> }`

			// TODO: define `auth` syntax and parse it
			let _fields = self.expect_braced(Self::parse_field_defs)?;

			Auth::Define {}
		};

		Ok((auth_name, kind))
	}

	#[tracing::instrument(level = "DEBUG", skip(self))]
	fn parse_body(&mut self) -> PResult<Body> {
		self.expect_keyword(kw::Body)?;
		let ty = self.parse_ty()?;
		Ok(Body { ty })
	}

	#[tracing::instrument(level = "DEBUG", skip(self))]
	fn parse_params(&mut self) -> PResult<Params> {
		self.expect_keyword(kw::Params)?;
		let properties = self.expect_braced(Self::parse_field_defs)?;
		Ok(Params { properties })
	}
}

#[cfg(test)]
mod tests {
	use crate::assert_tokenize;

	assert_tokenize!("simple", parse_path_item_kind, "var");
	assert_tokenize!("variable", parse_path_item_kind, "{var}");
	assert_tokenize!("complex", parse_path_item_kind, "var1/{var2}");
	assert_tokenize!("long_complex", parse_path_item_kind, "var1/{var2}/{var3}");
}
