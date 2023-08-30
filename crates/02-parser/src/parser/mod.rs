use crate::PResult;
use ast::types::Ident;
use lexer::{
	rich::{Enricher, Token, TokenKind},
	symbols::Symbol,
};
use session::ParseSession;
use std::mem;
use tracing::instrument;

mod attr;
mod expr;
mod factory;
mod item;

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
	pub session: &'a ParseSession,
	/// The current token.
	pub token: Token,
	/// The spacing for the current token
	pub token_spacing: Spacing,
	/// The previous token.
	pub prev_token: Token,

	expected_tokens: Vec<TokenKind>,
	cursor: Enricher<'a>,
}

impl<'a> Parser<'a> {
	#[must_use]
	pub fn from_source(session: &'a ParseSession, source: &'a str) -> Self {
		let tokens = Enricher::from_source(session, source);
		Self::from_tokens(session, tokens)
	}

	#[must_use]
	pub fn from_tokens(session: &'a ParseSession, cursor: Enricher<'a>) -> Self {
		let mut parser = Self {
			session,
			token: Token::DUMMY,
			token_spacing: Spacing::Alone,
			prev_token: Token::DUMMY,
			expected_tokens: Vec::default(),
			cursor,
		};

		parser.bump();

		parser
	}

	/// Expects and consumes the token `t`. Signals an error if the next token is not `t`.
	#[track_caller]
	#[instrument(level = "TRACE", skip(self))]
	pub fn expect(&mut self, tok: &TokenKind) -> PResult</* recovered */ bool> {
		if self.expected_tokens.is_empty() {
			if self.token.kind == *tok {
				self.bump();
				Ok(false)
			} else {
				todo!("recover from unexpected token {}", self.token)
			}
		} else {
			if self.token.kind == *tok {
				self.bump();
				Ok(false)
			} else {
				todo!("recover from unexpected token {}", self.token)
			}
			// todo!("understand why branch here")
			// self.expect_one_of(slice::from_ref(tok), &[])
		}
	}

	/// If the given word is not a keyword, signals an error.
	/// If the next token is not the given word, signals an error.
	/// Otherwise, eats it.
	#[instrument(level = "TRACE", skip(self))]
	fn expect_keyword(&mut self, kw: Symbol) -> PResult<()> {
		if self.eat_keyword(kw) {
			Ok(())
		} else {
			todo!()
		}
	}

	#[instrument(level = "TRACE", skip(self))]
	fn bump(&mut self) {
		use Spacing::*;
		let (next_token, has_space_before) = self.cursor.next_token();

		self.prev_token = mem::replace(&mut self.token, next_token);
		self.token_spacing = if has_space_before { Alone } else { Joint };

		self.expected_tokens.clear();
	}

	#[instrument(level = "TRACE", skip(self))]
	fn check(&mut self, tok: &TokenKind) -> bool {
		let is_present = self.token.kind == *tok;

		if !is_present {
			self.expected_tokens.push(tok.clone());
		}

		is_present
	}

	/// Consumes a token 'tok' if it exists. Returns whether the given token was present.
	#[instrument(level = "TRACE", skip(self))]
	pub fn eat(&mut self, tok: &TokenKind) -> bool {
		let is_present = self.check(tok);
		if is_present {
			self.bump();
		}
		is_present
	}

	/// If the next token is the given keyword, returns `true` without eating it.
	/// An expectation is also added for diagnostics purposes.
	#[instrument(level = "TRACE", skip(self))]
	fn check_keyword(&mut self, kw: Symbol) -> bool {
		self.expected_tokens.push(TokenKind::Ident(kw));
		self.token.is_keyword(kw)
	}

	/// If the next token is the given keyword, eats it and returns `true`.
	/// Otherwise, returns `false`. An expectation is also added for diagnostics purposes.
	#[instrument(level = "TRACE", skip(self))]
	pub fn eat_keyword(&mut self, kw: Symbol) -> bool {
		if self.check_keyword(kw) {
			self.bump();
			true
		} else {
			false
		}
	}

	#[instrument(level = "TRACE", skip(self))]
	fn parse_ident(&mut self) -> PResult<Ident> {
		let ident = if let Some(lexer::symbols::Ident { symbol, span }) = self.token.ident() {
			Ident { symbol, span }
		} else {
			let recover = true;
			if recover {
				todo!()
			} else {
				return Err(todo!());
			}
		};

		self.bump();
		Ok(ident)
	}
}
