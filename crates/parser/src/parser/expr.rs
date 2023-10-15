use crate::{error::UnexpectedToken, PResult, Parser};
use ast::types::{AttrStyle, Expr, ExprKind, FieldDef, NodeId, PropertyDef, P};
use lexer::rich::{Delimiter, LiteralKind, TokenKind};
use session::{
	sym,
	symbols::{attrs, kw},
	Ident,
};
use thin_vec::{thin_vec, ThinVec};
use tracing::instrument;

impl<'a> Parser<'a> {
	#[instrument(level = "DEBUG", skip(self))]
	pub(super) fn parse_expr(&mut self) -> PResult<P<Expr>> {
		let lo = self.token.span;

		let kind = self.parse_expr_kind()?;

		Ok(Self::make_expr(thin_vec![], kind, self.span(lo)))
	}

	#[instrument(level = "DEBUG", skip(self))]
	fn parse_expr_kind(&mut self) -> PResult<ExprKind> {
		if self.check(&TokenKind::OpenDelim(Delimiter::Bracket)) {
			self.parse_expr_array()
		} else {
			self.parse_expr_literal()
		}
	}

	#[instrument(level = "DEBUG", skip(self))]
	fn parse_expr_literal(&mut self) -> PResult<ExprKind> {
		if let TokenKind::Literal(kind, sym) = self.token.kind {
			self.bump();
			Ok(ExprKind::Literal(kind, sym))
		} else if let Some(Ident {
			symbol: sym @ (kw::True | kw::False),
			..
		}) = self.token.ident().map(Into::into)
		{
			self.bump();
			Ok(ExprKind::Literal(LiteralKind::Bool, sym))
		} else {
			// TODO: recover properly
			Err(UnexpectedToken {
				parsed: self.token.clone(),
				expected: TokenKind::Literal(LiteralKind::Str, sym!("remove")),
			}
			.into())
		}
	}

	#[instrument(level = "DEBUG", skip(self))]
	fn parse_expr_array(&mut self) -> PResult<ExprKind> {
		let mut items = ThinVec::default();

		self.expect(&TokenKind::OpenDelim(Delimiter::Bracket))?;
		while !self.eat(&TokenKind::CloseDelim(Delimiter::Bracket)) {
			items.push(self.parse_expr()?);
		}

		Ok(ExprKind::Array(items))
	}

	#[instrument(level = "DEBUG", skip(self))]
	pub(super) fn parse_property_defs(&mut self) -> PResult<ThinVec<P<PropertyDef>>> {
		let mut props = ThinVec::new();
		while let Some(prop) = self.parse_property_def()? {
			props.push(prop);
		}
		Ok(props)
	}

	#[instrument(level = "DEBUG", skip(self))]
	fn parse_property_def(&mut self) -> PResult<Option<P<PropertyDef>>> {
		let lo = self.token.span;

		let mut attrs = self.parse_outer_attrs()?;

		let Some(ident) = self.eat_ident() else {
			return Ok(None);
		};
		let value = self.parse_expr()?;

		if let Some(attrs_) = self.parse_inline_attrs()? {
			attrs.extend(attrs_);
		}

		Ok(Some(Self::make_property_def(
			attrs,
			ident,
			value,
			self.span(lo),
		)))
	}

	#[instrument(level = "DEBUG", skip(self))]
	pub(super) fn parse_field_defs(&mut self) -> PResult<ThinVec<P<FieldDef>>> {
		let mut fields = ThinVec::new();
		while let Some(field) = self.parse_field_def()? {
			fields.push(field);
		}
		Ok(fields)
	}

	#[instrument(level = "DEBUG", skip(self))]
	fn parse_field_def(&mut self) -> PResult<Option<P<FieldDef>>> {
		let lo = self.token.span;

		let mut attrs = self.parse_outer_attrs()?;

		let Some(ident) = self.eat_ident() else {
			return Ok(None);
		};
		let ty = self.parse_ty()?;

		// Extract the optional literal after the type and transform into sugar for a meta attr named `description`
		if let Ok(ExprKind::Literal(LiteralKind::Str, sym)) = self.parse_expr_literal() {
			attrs.push(Self::make_meta_attr(
				Ident::new(attrs::description, self.prev_token.span),
				Some(P(Expr {
					attrs: ThinVec::new(),
					kind: ExprKind::Literal(LiteralKind::Str, sym),
					id: NodeId::DUMMY,
					span: self.prev_token.span,
				})),
				AttrStyle::Inline,
				self.prev_token.span,
			));
		}

		if let Some(inline_attrs) = self.parse_inline_attrs()? {
			attrs.extend(inline_attrs);
		}

		Ok(Some(Self::make_field_def(attrs, ident, ty, self.span(lo))))
	}
}

#[cfg(test)]
mod tests {
	use crate::assert_tokenize;

	assert_tokenize!(
		parse_field_defs,
		r#"
			## # Safety
			## This is a comment
			## This is a second line of comment
			Authorization long_string "The API Key of the User of the User" |@prefix: "Api-Key"|
			# ^ ident     ^ type      ^ sugar for description attr          ^ prefix attr

			X-Model string "The Model of the User"
		"#
	);
}
