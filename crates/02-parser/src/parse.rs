use crate::{PResult, Parser};
use ast::types::{Api, Attribute, Ident, Item, ItemKind, Metadata, ScopeKind};
use lexer::{
	rich::{Delimiter, TokenKind},
	span::Span,
	symbols::kw,
};

impl<'a> Parser<'a> {
	pub fn parse_root(&mut self) -> PResult<Api> {
		let attrs = self.parse_inner_attrs()?;
		let meta = self.parse_metadata()?;

		let items = self.parse_scope_content(&mut vec![])?;

		Ok(Api {
			attrs,
			meta,
			items,
			span: Span::dummy(),
		})
	}

	fn parse_scope_content(&mut self, attrs: &mut Vec<Attribute>) -> PResult<Vec<Item>> {
		attrs.extend(self.parse_inner_attrs()?);

		let mut items = Vec::default();
		while let Some(item) = self.parse_item()? {
			items.push(item);
		}

		Ok(items)
	}

	fn parse_inner_attrs(&mut self) -> PResult<Vec<Attribute>> {
		self.expect(&TokenKind::At)?;
		self.expect(&TokenKind::Bang)?;
		self.expect(&TokenKind::CloseDelim(Delimiter::Parenthesis))?;
		// TODO: eat attr content
		self.expect(&TokenKind::CloseDelim(Delimiter::Parenthesis))?;

		todo!()
	}

	fn parse_attrs(&mut self) -> PResult<Vec<Attribute>> {
		self.expect(&TokenKind::At)?;
		self.expect(&TokenKind::CloseDelim(Delimiter::Parenthesis))?;
		// TODO: eat attr content
		self.expect(&TokenKind::CloseDelim(Delimiter::Parenthesis))?;

		todo!()
	}

	fn parse_metadata(&mut self) -> PResult<Metadata> {
		if !self.eat_keyword(kw::Meta) {
			// wtf, react accordingly
		}

		self.expect(&TokenKind::OpenDelim(Delimiter::Brace))?;
		// TODO: eat metadata
		self.expect(&TokenKind::CloseDelim(Delimiter::Brace))?;

		todo!()
	}

	fn parse_item(&mut self) -> PResult<Option<Item>> {
		let attrs = self.parse_attrs()?;

		self.parse_item_(attrs)
	}

	fn parse_item_(&mut self, mut attrs: Vec<Attribute>) -> PResult<Option<Item>> {
		let (ident, kind) = if self.eat_keyword(kw::Scope) {
			self.parse_scope(&mut attrs)?
		} else {
			return Ok(None);
		};

		Ok(Some(Item {
			attrs,
			ident,
			kind,
			span: Span::dummy(),
		}))
	}

	fn parse_scope(&mut self, attrs: &mut Vec<Attribute>) -> PResult<(Ident, ItemKind)> {
		let ident = self.parse_ident()?;

		let scope = if self.eat(&TokenKind::Semi) {
			ScopeKind::Unloaded
		} else {
			self.expect(&TokenKind::OpenDelim(Delimiter::Brace))?;
			let items = self.parse_scope_content(attrs)?;
			self.expect(&TokenKind::CloseDelim(Delimiter::Brace))?;

			ScopeKind::Loaded {
				items,
				inline: true,
				span: Span::dummy(),
			}
		};

		Ok((ident, ItemKind::Scope(scope)))
	}

	fn parse_ident(&mut self) -> PResult<Ident> {
		todo!()
	}
}
