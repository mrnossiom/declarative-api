use crate::{PResult, Parser};
use ast::types::{AttrKind, AttrStyle, AttrVec, Attribute};
use lexer::{rich::TokenKind, span::Span};

impl<'a> Parser<'a> {
	pub(crate) fn parse_inner_attrs(&mut self) -> PResult<Vec<Attribute>> {
		self.parse_attrs(AttrStyle::Inner)
	}

	pub(crate) fn parse_outer_attrs(&mut self) -> PResult<Vec<Attribute>> {
		self.parse_attrs(AttrStyle::OuterOrInline)
	}

	fn parse_attrs(&mut self, style: AttrStyle) -> PResult<Vec<Attribute>> {
		let mut attrs = AttrVec::new();

		loop {
			let attr = if self.check(&TokenKind::Pound) {
				Some(self.parse_attr(style)?)
			} else if let TokenKind::DocComment(style, sym) = self.token.kind {
				Some(Attribute {
					kind: AttrKind::DocComment(sym),
					// TODO: gen attr id
					id: 0,
					style: style.into(),
					// TODO: set span
					span: Span::dummy(),
				})
			} else {
				None
			};

			if let Some(attr) = attr {
				if attr.style == style {
					attrs.push(attr);
				}
			} else {
				break;
			}
		}

		Ok(attrs)
	}

	fn parse_attr(&mut self, style: AttrStyle) -> PResult<Attribute> {
		self.expect(&TokenKind::At)?;

		let style_found = if self.eat(&TokenKind::Bang) {
			AttrStyle::Inner
		} else {
			AttrStyle::OuterOrInline
		};

		todo!("eat attr content")
	}
}
