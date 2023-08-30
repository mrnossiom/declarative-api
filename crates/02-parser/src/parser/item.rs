use crate::{PResult, Parser};
use ast::{
	types::{Api, AttrVec, Ident, Item, ItemKind, Metadata, NodeId, ScopeKind},
	P,
};
use lexer::{
	rich::{Delimiter, TokenKind},
	symbols::kw,
};

impl<'a> Parser<'a> {
	#[tracing::instrument(level = "DEBUG", skip(self))]
	pub fn parse_root(&mut self) -> PResult<Api> {
		let lo = self.token.span;

		let attrs = self.parse_inner_attrs()?;
		let meta = self.parse_metadata()?;

		let items = self.parse_scope_content(None)?;

		Ok(Api {
			attrs,
			meta,
			items,

			id: NodeId::ROOT,
			span: lo.to(self.prev_token.span),
		})
	}

	#[tracing::instrument(level = "DEBUG", skip(self, attrs))]
	fn parse_scope(&mut self, attrs: &mut AttrVec) -> PResult<(Ident, ItemKind)> {
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

	fn parse_scope_content(&mut self, attrs: Option<&mut AttrVec>) -> PResult<Vec<P<Item>>> {
		if let Some(attrs) = attrs {
			attrs.extend(self.parse_inner_attrs()?);
		}

		let mut items = Vec::default();
		while let Some(item) = self.parse_item()? {
			items.push(item);
		}

		Ok(items)
	}

	fn parse_metadata(&mut self) -> PResult<Metadata> {
		if !self.eat_keyword(kw::Meta) {
			// TODO: react accordingly
		}

		self.expect(&TokenKind::OpenDelim(Delimiter::Brace))?;
		// TODO: eat metadata
		self.expect(&TokenKind::CloseDelim(Delimiter::Brace))?;

		todo!()
	}

	fn parse_item(&mut self) -> PResult<Option<P<Item>>> {
		let attrs = self.parse_outer_attrs()?;

		self.parse_item_(attrs).map(|item| item.map(P::<Item>::new))
	}

	fn parse_item_(&mut self, mut attrs: AttrVec) -> PResult<Option<Item>> {
		let lo = self.token.span;

		let (ident, kind) = if self.eat_keyword(kw::Scope) {
			self.parse_scope(&mut attrs)?
		} else {
			return Ok(None);
		};

		Ok(Some(Item {
			attrs,
			ident,
			kind,

			id: NodeId::DUMMY,
			span: lo.to(self.prev_token.span),
		}))
	}
}
