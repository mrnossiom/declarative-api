use crate::{PResult, Parser};
use ast::{types::Type, P};
use tracing::instrument;

impl<'a> Parser<'a> {
	#[instrument(level = "DEBUG", skip(self))]
	pub(super) fn parse_ty(&mut self) -> PResult<P<Type>> {
		todo!()
	}
}
