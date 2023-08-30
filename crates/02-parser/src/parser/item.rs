use crate::{PResult, Parser};
use ast::{
	types::{Api, AttrVec, Ident, Item, ItemKind, Metadata, NodeId, Path, PathKind, ScopeKind},
	P,
};
use lexer::{
	rich::{Delimiter, OpKind, TokenKind},
	symbols::kw,
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
			self.parse_scope(&mut attrs)?
		} else if self.check_keyword(kw::Path) {
			self.parse_path_item()?
		} else {
			return Ok(None);
		};

		Ok(Some(Self::make_item(
			attrs,
			kind,
			Some(ident),
			self.span(lo),
		)))
	}

	#[tracing::instrument(level = "DEBUG", skip(self))]
	fn parse_path_item(&mut self) -> PResult<(Ident, ItemKind)> {
		self.expect_keyword(kw::Path)?;

		let (ident, kind) = self.parse_path_item_kind()?;

		self.expect(&TokenKind::OpenDelim(Delimiter::Brace))?;
		// TODO: get path items
		let items = thin_vec![];
		self.expect(&TokenKind::CloseDelim(Delimiter::Brace))?;

		Ok((ident, ItemKind::Path(Path { kind, items })))
	}

	#[tracing::instrument(level = "DEBUG", skip(self))]
	fn parse_path_item_kind(&mut self) -> PResult<(Ident, PathKind)> {
		if self.eat(&TokenKind::OpenDelim(Delimiter::Brace)) {
			let ident = self.parse_ident()?;
			self.expect(&TokenKind::CloseDelim(Delimiter::Brace))?;

			if self.eat(&TokenKind::Op(OpKind::Slash)) {
				let mut parts = thin_vec![];

				match self.parse_path_item_kind()?.1 {
					PathKind::Complex(vec) => parts.extend(vec),
					pk @ (PathKind::Simple(..) | PathKind::Variable(..)) => parts.push(pk),
				}

				Ok((ident, PathKind::Complex(parts)))
			} else {
				Ok((ident.clone(), PathKind::Variable(ident)))
			}
		} else {
			let ident = self.parse_ident()?;

			Ok((ident.clone(), PathKind::Simple(ident)))
		}
	}
}
