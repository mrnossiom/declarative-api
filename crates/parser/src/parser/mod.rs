use crate::{
	error::{UnexpectedToken, UnexpectedTokenInsteadOfKeyword},
	PResult,
};
use dapic_lexer::rich::{Delimiter, Enricher, Token, TokenKind};
use dapic_session::{DiagnosticsHandler, Ident, ParseSession, SourceFile, Symbol};
use std::mem;
use thin_vec::ThinVec;
use tracing::instrument;

mod attr;
mod expr;
mod factory;
mod item;
mod ty;

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
	pub fn from_source(session: &'a ParseSession, source: &'a SourceFile) -> Self {
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

	#[must_use]
	const fn diag(&self) -> &DiagnosticsHandler {
		&self.session.diagnostic
	}

	/// Expects and consumes the token `t`. Signals an error if the next token is not `t`.
	#[track_caller]
	#[instrument(level = "TRACE", skip(self))]
	fn expect(&mut self, tok: &TokenKind) -> PResult</* recovered */ bool> {
		if self.expected_tokens.is_empty() {
			if &self.token.kind == tok {
				self.bump();
				Ok(false)
			} else {
				// todo!("recover from unexpected token {}", self.token)
				Err(UnexpectedToken {
					parsed: self.token.clone(),
					expected: *tok,
				}
				.into())
			}
		} else if &self.token.kind == tok {
			self.bump();
			Ok(false)
		} else {
			// todo!("recover from unexpected token {}", self.token)

			// TODO: pass `expected_tokens`

			Err(UnexpectedToken {
				parsed: self.token.clone(),
				expected: *tok,
			}
			.into())
		}
	}

	#[track_caller]
	fn expect_braced<T>(&mut self, mut p: impl FnMut(&mut Self) -> PResult<T>) -> PResult<T> {
		self.expect(&TokenKind::OpenDelim(Delimiter::Brace))?;
		let parsed = p(self)?;
		self.expect(&TokenKind::CloseDelim(Delimiter::Brace))?;
		Ok(parsed)
	}

	/// If the given word is not a keyword, signals an error.
	/// If the next token is not the given word, signals an error.
	/// Otherwise, eats it.
	#[instrument(level = "TRACE", skip(self))]
	fn expect_keyword(&mut self, kw: Symbol) -> PResult<()> {
		if self.eat_keyword(kw) {
			Ok(())
		} else {
			Err(UnexpectedTokenInsteadOfKeyword {
				parsed: self.token.clone(),
				expected: kw,
			}
			.into())
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
		let is_present = &self.token.kind == tok;

		if !is_present {
			self.expected_tokens.push(*tok);
		}

		is_present
	}

	/// Consumes a token 'tok' if it exists. Returns whether the given token was present.
	#[instrument(level = "TRACE", skip(self))]
	fn eat(&mut self, tok: &TokenKind) -> bool {
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
	fn eat_keyword(&mut self, kw: Symbol) -> bool {
		if self.check_keyword(kw) {
			self.bump();
			true
		} else {
			false
		}
	}

	/// If the next token is the given keyword, returns `true` without eating it.
	/// An expectation is also added for diagnostics purposes.
	#[instrument(level = "TRACE", skip(self))]
	fn check_ident(&mut self) -> bool {
		// self.expected_tokens.push(TokenKind::Ident(kw));
		self.token.ident().is_some()
	}

	#[instrument(level = "TRACE", skip(self))]
	fn eat_ident(&mut self) -> Option<Ident> {
		self.token.ident().map(|ident| {
			self.bump();
			ident
		})
	}

	#[instrument(level = "TRACE", skip(self))]
	fn parse_ident(&mut self) -> PResult<Ident> {
		let Some(ident) = self.token.ident() else {
			todo!("recover")
		};

		self.bump();
		Ok(ident)
	}

	#[instrument(level = "TRACE", skip(self))]
	fn parse_delimited(&mut self) -> PResult<(Delimiter, ThinVec<Token>)> {
		let mut tokens = ThinVec::default();

		let delim_kind = match self.token.kind {
			TokenKind::OpenDelim(delim) => {
				self.bump();
				delim
			}
			_ => todo!("recover"),
		};

		let mut nesting = 0;

		loop {
			match self.token.kind {
				TokenKind::OpenDelim(delim) if delim == delim_kind => {
					nesting += 1;
				}

				TokenKind::CloseDelim(delim) if delim == delim_kind => {
					if nesting == 0 {
						break Ok((delim_kind, tokens));
					}

					nesting -= 1;
				}
				_ => {}
			}

			tokens.push(self.token.clone());
			self.bump();
		}
	}
}

#[cfg(test)]
mod tests {

	#[macro_export]
	macro_rules! parser {
		($name:ident; $src:literal) => {
			let src = $src;
			$crate::parser!($name; src)
		};
		($name:ident; $src:ident) => {
			let session = dapic_session::ParseSession::default();
			let sf = session.source_map.load_anon($src.into());
			let mut $name = $crate::Parser::from_source(&session, &sf);
		};
	}

	#[macro_export]
	macro_rules! assert_tokenize {
		($method:ident, $src:literal) => {
			#[test]
			fn $method() -> Result<(), dapic_session::Diagnostic> {
				$crate::parser!(p; $src);
				insta::assert_debug_snapshot!(p.$method()?);
				Ok(())
			}
		};
		($method:ident, $variant:literal, $src:literal) => {
			paste::paste! {
				#[test]
				fn [<$method _ $variant>]() -> Result<(), dapic_session::Diagnostic> {
					$crate::parser!(p; $src);
					insta::assert_debug_snapshot!(p.$method()?);
					Ok(())
				}
			}
		};
	}

	assert_tokenize!(parse_delimited, "(bar, baz)");
}
