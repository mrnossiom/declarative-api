use crate::{PError, PResult, Parser};
use ast::types::{AttrId, AttrKind, AttrStyle, AttrVec, Attribute};
use lexer::rich::TokenKind;

impl<'a> Parser<'a> {
	pub(crate) fn parse_inner_attrs(&mut self) -> PResult<AttrVec> {
		self.parse_attrs(AttrStyle::Inner)
	}

	pub(crate) fn parse_outer_attrs(&mut self) -> PResult<AttrVec> {
		self.parse_attrs(AttrStyle::OuterOrInline)
	}

	fn parse_attrs(&mut self, style: AttrStyle) -> PResult<AttrVec> {
		let mut attrs = AttrVec::new();

		loop {
			let attr = if self.check(&TokenKind::Pound) {
				Some(self.parse_attr(style)?)
			} else if let TokenKind::DocComment(style, sym) = self.token.kind {
				Some(Attribute {
					kind: AttrKind::DocComment(sym),
					id: AttrId::make_id(),
					style: style.into(),
					// TODO: set span
					span: self.token.span,
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

		if style_found == style {
			return Err(PError::new(format!(
				"found a {style_found} styled attribute where we expected a {style} styled attribute"
			)));
		}

		todo!("eat attr content")
	}
}
