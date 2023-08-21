pub use crate::error::*;
use lexer::rich::{Enricher, Token};

pub struct Parser<'a> {
	// pub sess: &'a ParseSess,
	/// The current token.
	pub token: Token,
	/// The spacing for the current token
	// pub token_spacing: Spacing,
	/// The previous token.
	pub prev_token: Token,
	pub capture_cfg: bool,

	// expected_tokens: Vec<TokenType>,
	// Important: This must only be advanced from `bump` to ensure that
	// `token_cursor.num_next_calls` is updated properly.
	token_cursor: Enricher<'a>,
}

impl<'a> Parser<'a> {
	pub fn from_tokens() -> Self {
		todo!()
	}
}
