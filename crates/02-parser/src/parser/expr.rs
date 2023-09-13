use crate::{PResult, Parser};
use ast::{
	types::{AttrStyle, Expr, ExprKind, FieldDef, NodeId, PropertyDef},
	P,
};
use lexer::rich::{Delimiter, LiteralKind, TokenKind};
use session::{
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
			todo!("")
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

		// TODO: parse inline attrs
		attrs.extend(ThinVec::new());

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
				Ident::new(attrs::Description, self.prev_token.span),
				Some(P(Expr {
					attrs: ThinVec::new(),
					kind: ExprKind::Literal(LiteralKind::Str, sym),
					id: NodeId::DUMMY,
					span: self.prev_token.span,
				})),
				AttrStyle::OuterOrInline,
				self.prev_token.span,
			));
		}

		// TODO: parse inline attrs
		attrs.extend(ThinVec::new());

		Ok(Some(Self::make_field_def(attrs, ident, ty, self.span(lo))))
	}
}

#[cfg(test)]
mod tests {
	use crate::Parser;
	use ast::types::{AttrStyle, NodeId, Path, PathSegment, TyKind};
	use session::{ident, sp, sym, ParseSession};
	use std::error::Error;
	use thin_vec::thin_vec;

	#[test]
	fn parse_field_defs() -> Result<(), Box<dyn Error>> {
		let src = r#"
## # Safety
## This is a comment
## This is a second line of comment
Authorization long_string "The API Key of the User of the User" @prefix: "Api-Key"
# ^ ident     ^ type      ^ sugar for description attr          ^ prefix attr

X-Model string "The Model of the User"
"#;

		let session = ParseSession::default();
		let source = session.source_map.load_anon(src.into());
		let mut p = Parser::from_source(&session, &source);

		let fields = p.parse_field_defs()?;

		assert_eq!(fields.len(), 2);

		assert_eq!(
			fields[0],
			Parser::make_field_def(
				thin_vec![Parser::make_doc_attr(
					sym!(" # Safety"),
					AttrStyle::OuterOrInline,
					sp!(0, 0)
				)],
				ident!("Authorization", 0, 13),
				Parser::make_ty(
					TyKind::Path(Path {
						segments: thin_vec![PathSegment {
							ident: ident!("long_string", 15, 26),
							id: NodeId::DUMMY
						}],
						span: sp!(0, 13),
					}),
					sp!(0, 13)
				),
				sp!(0, 20)
			)
		);

		assert_eq!(
			fields[0],
			Parser::make_field_def(
				thin_vec![],
				ident!("X-Model", 0, 13),
				Parser::make_ty(
					Parser::make_ty_kind_single(ident!("string", 15, 26), sp!(0, 13)),
					sp!(0, 0)
				),
				sp!(0, 20)
			)
		);

		Ok(())
	}
}