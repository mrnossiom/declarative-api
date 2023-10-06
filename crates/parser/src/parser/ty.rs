use crate::{error::UnexpectedToken, PResult, Parser};
use ast::{Ty, TyKind, P};
use lexer::rich::{Delimiter, TokenKind};
use session::sym;
use thin_vec::thin_vec;
use tracing::instrument;

impl<'a> Parser<'a> {
	#[instrument(level = "DEBUG", skip(self))]
	pub(super) fn parse_ty(&mut self) -> PResult<P<Ty>> {
		let lo = self.token.span;

		let kind = if self.check(&TokenKind::OpenDelim(Delimiter::Parenthesis)) {
			self.parse_ty_tuple_or_paren()?
		} else if self.check(&TokenKind::OpenDelim(Delimiter::Bracket)) {
			self.parse_ty_array()?
		} else if self.check(&TokenKind::OpenDelim(Delimiter::Brace)) {
			self.parse_ty_inline_model()?
		} else if let Some(ident) = self.eat_ident() {
			Self::make_ty_kind_single(ident, self.token.span)
		} else {
			// TODO: maybe recover?
			return Err(UnexpectedToken {
				parsed: self.token.clone(),
				expected: TokenKind::Ident(sym!("Type")),
			}
			.into());
		};

		Ok(Self::make_ty(kind, self.span(lo)))
	}

	#[tracing::instrument(level = "DEBUG", skip(self))]
	fn parse_ty_tuple_or_paren(&mut self) -> PResult<TyKind> {
		self.expect(&TokenKind::OpenDelim(Delimiter::Parenthesis))?;

		let ty = self.parse_ty()?;

		let kind = if self.eat(&TokenKind::Comma) {
			let mut tys = thin_vec![ty];

			loop {
				match self.parse_ty() {
					Ok(ty) => tys.push(ty),
					Err(_) => break,
				};

				if !self.eat(&TokenKind::Comma) {
					break;
				}
			}

			TyKind::Tuple(tys)
		} else {
			TyKind::Paren(ty)
		};

		self.expect(&TokenKind::CloseDelim(Delimiter::Parenthesis))?;

		Ok(kind)
	}

	#[tracing::instrument(level = "DEBUG", skip(self))]
	fn parse_ty_array(&mut self) -> PResult<TyKind> {
		self.expect(&TokenKind::OpenDelim(Delimiter::Bracket))?;
		let ty = self.parse_ty()?;
		self.expect(&TokenKind::CloseDelim(Delimiter::Bracket))?;
		Ok(TyKind::Array(ty))
	}

	#[tracing::instrument(level = "DEBUG", skip(self))]
	fn parse_ty_inline_model(&mut self) -> PResult<TyKind> {
		let fields = self.expect_braced(Self::parse_field_defs)?;
		Ok(TyKind::InlineModel(fields))
	}
}
