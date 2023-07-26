use ast::ast::{Api, Attribute, Item, Symbol};

mod error;

use error::PResult;

struct Parser {}

impl Parser {
	pub fn parse_api(&mut self) -> PResult<Api> {
		// Should be the first item in the file
		let metadata = self.parse_metadata()?;

		let mut items = Vec::default();
		while let Some(item) = self.parse_item()? {
			items.push(item);
		}

		Ok(Api {
			attrs,
			items,
			span: (),
			id: (),
		})
	}

	fn parse_metadata(&mut self) -> PResult<Vec<Attribute>> {
		todo!()
	}

	fn parse_item(&mut self) -> PResult<Item> {
		if self.eat_keyword(kw::Scope) {
			self.parse_scope()
		}

		todo!()
	}

	fn parse_scope(&mut self) -> PResult<Item> {
		todo!()
	}

	/// If the next token is the given keyword, eats it and returns `true`.
	/// Otherwise, returns `false`. An expectation is also added for diagnostics purposes.
	// Public for rustfmt usage.
	pub fn eat_keyword(&mut self, kw: Symbol) -> bool {
		if self.check_keyword(kw) {
			self.bump();
			true
		} else {
			false
		}
	}
}
