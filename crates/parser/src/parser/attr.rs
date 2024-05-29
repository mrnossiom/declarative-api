use crate::{error::WrongAttrStyle, PResult, Parser};
use dapic_ast::types::{AttrStyle, AttrVec, Attribute};
use dapic_lexer::rich::{self, OpKind, TokenKind};
use tracing::{debug_span, instrument};

impl<'a> Parser<'a> {
	/// Parse `<inner_attrs>`
	#[instrument(level = "DEBUG", skip(self))]
	pub(super) fn parse_inner_attrs(&mut self) -> PResult<AttrVec> {
		self.parse_attrs(AttrStyle::Inner)
	}

	/// Parse `<outer_attrs>`
	#[instrument(level = "DEBUG", skip(self))]
	pub(super) fn parse_outer_attrs(&mut self) -> PResult<AttrVec> {
		self.parse_attrs(AttrStyle::Outer)
	}

	/// Parse `|<inline_attrs>|`
	#[instrument(level = "DEBUG", skip(self))]
	pub(super) fn parse_inline_attrs(&mut self) -> PResult<Option<AttrVec>> {
		if self.eat(&TokenKind::Op(OpKind::Or)) {
			let attrs = self.parse_attrs(AttrStyle::Inline)?;
			self.expect(&TokenKind::Op(OpKind::Or))?;
			Ok(Some(attrs))
		} else {
			Ok(None)
		}
	}

	#[instrument(level = "DEBUG", skip(self))]
	fn parse_attrs(&mut self, style: AttrStyle) -> PResult<AttrVec> {
		let mut attrs = AttrVec::new();

		loop {
			let attr = if self.check(&TokenKind::At) {
				Some(self.parse_attr()?)
			} else if let TokenKind::DocComment(style, sym) = self.token.kind {
				let _span = debug_span!("parse_doc_attr").entered();

				let style = match style {
					rich::DocStyle::Inner => AttrStyle::Inner,
					rich::DocStyle::Outer => AttrStyle::Outer,
				};

				self.bump();
				Some(Self::make_doc_attr(sym, style, self.prev_token.span))
			} else {
				None
			};

			if let Some(mut attr) = attr {
				if attr.style != style {
					// Emit and recover
					self.session.diag.emit(WrongAttrStyle {
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
		} else if self.eat(&TokenKind::At) {
			AttrStyle::Outer
		} else {
			AttrStyle::Inline
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

			// TODO: change parsing from an ident to a path to allow more complex
			// resolution of attributes that can process tokens from foreign apis

			let (delim, tokens) = self.parse_delimited()?;

			Ok(Self::make_normal_attr(
				ident,
				delim,
				tokens,
				style,
				self.span(lo),
			))
		} else {
			// Parse `@key`
			Ok(Self::make_meta_attr(ident, None, style, self.span(lo)))
		}
	}
}
