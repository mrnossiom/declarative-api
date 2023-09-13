use crate::{error::UnexpectedToken, PResult, Parser};
use ast::{
	types::{Ty, TyKind},
	P,
};
use lexer::rich::{Delimiter, TokenKind};
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
		} else if let Some(ident) = self.eat_ident() {
			Self::make_ty_kind_single(ident, self.token.span)
		} else {
			// TODO: maybe recover?
			return Err(UnexpectedToken {
				token: self.token.span,
				parsed: self.token.kind.clone(),
				expected: TokenKind::At,
			}
			.into());
		};

		Ok(Self::make_ty(kind, self.span(lo)))
	}

	fn parse_ty_tuple_or_paren(&mut self) -> PResult<TyKind> {
		self.expect(&TokenKind::OpenDelim(Delimiter::Parenthesis))?;

		let ty = self.parse_ty()?;

		let kind = if self.eat(&TokenKind::Comma) {
			// TODO: parse the rest of the tuple
			TyKind::Tuple(thin_vec![ty])
		} else {
			TyKind::Paren(ty)
		};

		self.expect(&TokenKind::CloseDelim(Delimiter::Parenthesis))?;

		Ok(kind)
	}

	fn parse_ty_array(&mut self) -> PResult<TyKind> {
		self.expect(&TokenKind::OpenDelim(Delimiter::Bracket))?;

		let ty = self.parse_ty()?;

		self.expect(&TokenKind::OpenDelim(Delimiter::Bracket))?;

		Ok(TyKind::Array(ty))
	}
}
