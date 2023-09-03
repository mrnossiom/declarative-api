use crate::{PResult, Parser};
use ast::{
	types::{Api, AttrVec, Headers, Item, ItemKind, Metadata, NodeId, Path, PathKind, ScopeKind},
	P,
};
use lexer::rich::{Delimiter, OpKind, TokenKind};
use session::{symbols::kw, Ident};
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
	fn parse_scope(&mut self, attrs: &mut AttrVec) -> PResult<(Ident, ItemKind)> {
		self.expect_keyword(kw::Scope)?;

		let ident = self.parse_ident()?;

		let scope = if self.eat(&TokenKind::Semi) {
			ScopeKind::Unloaded
		} else {
			let lo = self.token.span;
			self.expect(&TokenKind::OpenDelim(Delimiter::Brace))?;
			let items = self.parse_scope_content(Some(attrs))?;
			self.expect(&TokenKind::CloseDelim(Delimiter::Brace))?;

			ScopeKind::Loaded {
				items,
				inline: true,
				span: lo.to(self.prev_token.span),
			}
		};

		Ok((ident, ItemKind::Scope(scope)))
	}

	#[tracing::instrument(level = "DEBUG", skip(self, attrs))]
	fn parse_scope_content(&mut self, attrs: Option<&mut AttrVec>) -> PResult<ThinVec<P<Item>>> {
		if let Some(attrs) = attrs {
			attrs.extend(self.parse_inner_attrs()?);
		}

		let mut items = ThinVec::default();
		while let Some(item) = self.parse_item()? {
			items.push(item);
		}

		Ok(items)
	}

	#[tracing::instrument(level = "DEBUG", skip(self))]
	fn parse_metadata(&mut self) -> PResult<P<Item>> {
		let lo = self.token.span;

		if !self.eat_keyword(kw::Meta) {
			// TODO: react accordingly
		}

		self.expect(&TokenKind::OpenDelim(Delimiter::Brace))?;
		let fields = self.parse_property_defs()?;
		self.expect(&TokenKind::CloseDelim(Delimiter::Brace))?;

		let meta = Metadata { fields };

		Ok(Self::make_item(
			ThinVec::default(),
			ItemKind::Meta(meta),
			None,
			self.span(lo),
		))
	}

	#[tracing::instrument(level = "DEBUG", skip(self))]
	fn parse_item(&mut self) -> PResult<Option<P<Item>>> {
		let attrs = self.parse_outer_attrs()?;

		self.parse_item_(attrs)
	}

	#[tracing::instrument(level = "DEBUG", skip(self, attrs))]
	fn parse_item_(&mut self, mut attrs: AttrVec) -> PResult<Option<P<Item>>> {
		let lo = self.token.span;

		let (ident, kind) = if self.check_keyword(kw::Scope) {
			let (ident, kind) = self.parse_scope(&mut attrs)?;
			(Some(ident), kind)
		} else if self.check_keyword(kw::Path) {
			(None, self.parse_path_item()?)
		} else if self.check_keyword(kw::Meta) {
			// TODO: change or emit warn about misplaces metadata
			(None, self.parse_metadata()?.kind.clone())
		} else if self.check_keyword(kw::Headers) {
			(None, self.parse_headers()?)
		} else if true {
			todo!()
		} else {
			return Ok(None);
		};

		Ok(Some(Self::make_item(attrs, kind, ident, self.span(lo))))
	}

	#[tracing::instrument(level = "DEBUG", skip(self))]
	fn parse_path_item(&mut self) -> PResult<ItemKind> {
		self.expect_keyword(kw::Path)?;

		let kind = self.parse_path_item_kind()?;

		self.expect(&TokenKind::OpenDelim(Delimiter::Brace))?;
		// TODO: get path items
		let items = thin_vec![];
		self.expect(&TokenKind::CloseDelim(Delimiter::Brace))?;

		Ok(ItemKind::Path(Path { kind, items }))
	}

	#[tracing::instrument(level = "DEBUG", skip(self))]
	fn parse_path_item_kind(&mut self) -> PResult<PathKind> {
		let kind = if self.eat(&TokenKind::OpenDelim(Delimiter::Brace)) {
			let ident = self.parse_ident()?;
			self.expect(&TokenKind::CloseDelim(Delimiter::Brace))?;
			PathKind::Variable(ident)
		} else {
			let ident = self.parse_ident()?;

			PathKind::Simple(ident)
		};

		if self.eat(&TokenKind::Op(OpKind::Slash)) {
			let mut parts = thin_vec![kind];

			match self.parse_path_item_kind()? {
				PathKind::Complex(vec) => parts.extend(vec),
				pk @ (PathKind::Simple(..) | PathKind::Variable(..)) => parts.push(pk),
			}

			Ok(PathKind::Complex(parts))
		} else {
			Ok(kind)
		}
	}

	#[tracing::instrument(level = "DEBUG", skip(self))]
	fn parse_headers(&mut self) -> PResult<ItemKind> {
		self.expect_keyword(kw::Headers)?;

		self.expect(&TokenKind::OpenDelim(Delimiter::Brace))?;
		let headers = self.parse_field_defs()?;
		self.expect(&TokenKind::CloseDelim(Delimiter::Brace))?;

		Ok(ItemKind::Headers(Headers { headers }))
	}
}

#[cfg(test)]
mod tests {
	use ast::types::PathKind::{self, *};
	use session::{BytePos, Ident, Session, Span, Symbol};
	use std::error::Error;
	use thin_vec::thin_vec;

	macro_rules! ident {
		($name:literal, $start:literal, $end:literal) => {
			Ident::new(
				Symbol::intern($name),
				Span {
					start: BytePos($start),
					end: BytePos($end),
				},
			)
		};
	}

	fn expect_path_item(source: &str, expected: &PathKind) -> Result<(), Box<dyn Error>> {
		let session = Session::default();
		let source = session.parse.source_map.add_source(source.into());
		let mut p = crate::Parser::from_source(&session.parse, &source);

		let kind = p.parse_path_item_kind()?;

		assert_eq!(&kind, expected);

		Ok(())
	}

	#[test]
	fn parse_path_items_simple() -> Result<(), Box<dyn Error>> {
		expect_path_item("var", &Simple(ident!("var", 0, 3)))?;

		Ok(())
	}

	#[test]
	fn parse_path_items_variable() -> Result<(), Box<dyn Error>> {
		expect_path_item("{var}", &Variable(ident!("var", 1, 4)))?;
		Ok(())
	}

	#[test]
	fn parse_path_items_complex_mixed() -> Result<(), Box<dyn Error>> {
		expect_path_item(
			"var1/{var2}",
			&Complex(thin_vec![
				Simple(ident!("var1", 0, 4)),
				Variable(ident!("var2", 6, 10))
			]),
		)?;

		Ok(())
	}

	#[test]
	fn parse_path_items_complex_long() -> Result<(), Box<dyn Error>> {
		expect_path_item(
			"var1/var2/var3/var4",
			&Complex(thin_vec![
				Simple(ident!("var1", 0, 4)),
				Simple(ident!("var2", 5, 9)),
				Simple(ident!("var3", 10, 14)),
				Simple(ident!("var4", 15, 19)),
			]),
		)?;

		Ok(())
	}
}
