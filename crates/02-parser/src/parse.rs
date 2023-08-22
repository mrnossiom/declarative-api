use crate::{PResult, Parser};
use ast::ast::{Api, Attribute, Item};
use lexer::{span::Span, symbols::kw};

impl<'a> Parser<'a> {
	pub fn parse_api(&mut self) -> PResult<Api> {
		// Should be the first item in the file
		let metadata = self.parse_metadata()?;

		let mut items = Vec::default();
		while let Some(item) = self.parse_item()? {
			items.push(item);
		}

		Ok(Api {
			meta: metadata,
			items,
			span: Span { start: 0, end: 0 },
		})
	}

	fn parse_metadata(&mut self) -> PResult<Vec<Attribute>> {
		todo!()
	}

	fn parse_item(&mut self) -> PResult<Option<Item>> {
		if self.eat_keyword(kw::Scope) {
			self.parse_scope()?;
		}

		todo!()
	}

	fn parse_scope(&mut self) -> PResult<Item> {
		todo!()
	}
}
