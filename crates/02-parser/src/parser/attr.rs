use crate::{error::WrongAttrStyle, PResult, Parser};
use ast::types::{AttrStyle, AttrVec, Attribute};
use lexer::rich::TokenKind;
use tracing::{debug_span, instrument};

impl<'a> Parser<'a> {
	#[instrument(level = "DEBUG", skip(self))]
	pub(super) fn parse_inner_attrs(&mut self) -> PResult<AttrVec> {
		self.parse_attrs(AttrStyle::Inner)
	}

	#[instrument(level = "DEBUG", skip(self))]
	pub(super) fn parse_outer_attrs(&mut self) -> PResult<AttrVec> {
		self.parse_attrs(AttrStyle::OuterOrInline)
	}

	#[instrument(level = "DEBUG", skip(self))]
	fn parse_attrs(&mut self, style: AttrStyle) -> PResult<AttrVec> {
		let mut attrs = AttrVec::new();

		loop {
			let attr = if self.check(&TokenKind::At) {
				Some(self.parse_attr()?)
			} else if let TokenKind::DocComment(style, sym) = self.token.kind {
				let _span = debug_span!("parse_doc_attr").entered();

				self.bump();
				Some(Self::make_doc_attr(sym, style.into(), self.prev_token.span))
			} else {
				None
			};

			if let Some(mut attr) = attr {
				if attr.style != style {
					// Emit and recover
					self.diag().emit(WrongAttrStyle {
						attr: attr.span,
						style,
						parsed_style: attr.style,
					});

					attr.style = style;
				}

				// Emit wrong attr style to recover
				attrs.push(attr);
			} else {
				break;
			}
		}

		Ok(attrs)
	}

	#[instrument(level = "DEBUG", skip(self))]
	fn parse_attr(&mut self) -> PResult<Attribute> {
		let lo = self.token.span;

		self.expect(&TokenKind::At)?;

		let style = if self.eat(&TokenKind::Bang) {
			AttrStyle::Inner
		} else {
			AttrStyle::OuterOrInline
		};

		let ident = self.parse_ident()?;
		if self.eat(&TokenKind::Colon) {
			// Parse `@key: <value>`
			let expr = self.parse_expr()?;

			Ok(Self::make_meta_attr(
				ident,
				Some(expr),
				style,
				self.span(lo),
			))
		} else if self.token.is_open_delim() {
			// Parse `@key(<tokens>)`
			let (delim, tokens) = self.parse_delimited()?;

			Ok(Self::make_normal_attr(delim, tokens, style, self.span(lo)))
		} else {
			// Parse `@key`
			Ok(Self::make_meta_attr(ident, None, style, self.span(lo)))
		}
	}
}
