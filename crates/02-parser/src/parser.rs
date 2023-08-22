use lexer::{
	rich::{Enricher, Token, TokenKind},
	symbols::Symbol,
};
use std::mem;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spacing {
	/// The token is not immediately followed by an operator token (as
	/// determined by `Token::is_op`). E.g. a `+` token is `Alone` in `+ =`,
	/// `+/*foo*/=`, `+ident`, and `+()`.
	Alone,

	/// The token is immediately followed by an operator token. E.g. a `+`
	/// token is `Joint` in `+=` and `++`.
	Joint,
}

pub struct Parser<'a> {
	// pub sess: &'a ParseSess,
	/// The current token.
	pub token: Token,
	/// The spacing for the current token
	pub token_spacing: Spacing,
	/// The previous token.
	pub prev_token: Token,

	expected_tokens: Vec<TokenKind>,
	// Important: This must only be advanced from `bump` to ensure that
	// `token_cursor.num_next_calls` is updated properly.
	cursor: Enricher<'a>,
}

impl<'a> Parser<'a> {
	#[must_use]
	pub fn from_tokens(cursor: Enricher<'a>) -> Self {
		Self {
			token: Token::dummy(),
			token_spacing: Spacing::Alone,
			prev_token: Token::dummy(),
			expected_tokens: Vec::default(),
			cursor,
		}
	}

	fn bump(&mut self) {
		use Spacing::*;
		let (next_token, has_space_before) = self.cursor.next_token();

		self.prev_token = mem::replace(&mut self.token, next_token);
		self.token_spacing = if has_space_before { Alone } else { Joint };

		self.expected_tokens.clear();
	}

	fn check(&mut self, tok: &TokenKind) -> bool {
		let is_present = self.token.kind == *tok;

		if !is_present {
			self.expected_tokens.push(tok.clone());
		}

		is_present
	}

	/// Consumes a token 'tok' if it exists. Returns whether the given token was present.
	pub fn eat(&mut self, tok: &TokenKind) -> bool {
		let is_present = self.check(tok);
		if is_present {
			self.bump();
		}
		is_present
	}

	/// If the next token is the given keyword, returns `true` without eating it.
	/// An expectation is also added for diagnostics purposes.
	fn check_keyword(&mut self, kw: Symbol) -> bool {
		self.expected_tokens.push(TokenKind::Ident(kw));
		self.token.is_keyword(kw)
	}

	/// If the next token is the given keyword, eats it and returns `true`.
	/// Otherwise, returns `false`. An expectation is also added for diagnostics purposes.
	pub fn eat_keyword(&mut self, kw: Symbol) -> bool {
		if self.check_keyword(kw) {
			self.bump();
			true
		} else {
			false
		}
	}
}
