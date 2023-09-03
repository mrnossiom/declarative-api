use crate::{PResult, Parser};
use ast::{
	types::{Expr, ExprKind, FieldDef, PropertyDef},
	P,
};
use lexer::rich::{Delimiter, LiteralKind, TokenKind};
use session::{symbols::kw, Ident};
use thin_vec::{thin_vec, ThinVec};
use tracing::instrument;

impl<'a> Parser<'a> {
	#[instrument(level = "DEBUG", skip(self))]
	fn parse_expr(&mut self) -> PResult<P<Expr>> {
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
			todo!()
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

		while self.token.ident().is_some() {
			props.push(self.parse_property_def()?);
		}

		Ok(props)
	}

	#[instrument(level = "DEBUG", skip(self))]
	fn parse_property_def(&mut self) -> PResult<P<PropertyDef>> {
		let lo = self.token.span;

		let ident = self.parse_ident()?;
		let value = self.parse_expr()?;

		// TODO: parse inline attrs
		let attrs = ThinVec::new();

		Ok(Self::make_property_def(attrs, ident, value, self.span(lo)))
	}

	#[instrument(level = "DEBUG", skip(self))]
	pub(super) fn parse_field_defs(&mut self) -> PResult<ThinVec<P<FieldDef>>> {
		let mut props = ThinVec::new();

		while self.token.ident().is_some() {
			props.push(self.parse_field_def()?);
		}

		Ok(props)
	}

	#[instrument(level = "DEBUG", skip(self))]
	fn parse_field_def(&mut self) -> PResult<P<FieldDef>> {
		let lo: session::Span = self.token.span;

		let ident = self.parse_ident()?;
		let ty = self.parse_ty()?;

		// TODO: parse inline attrs
		let attrs = ThinVec::new();

		Ok(Self::make_field_def(attrs, ident, ty, self.span(lo)))
	}
}
