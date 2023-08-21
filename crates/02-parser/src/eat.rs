use crate::parser::Parser;
use lexer::{rich::TokenKind, symbols::Symbol};

impl<'a> Parser<'a> {
	fn check(&mut self, tok: &TokenKind) -> bool {
		let is_present = self.token == *tok;
		if !is_present {
			self.expected_tokens.push(TokenType::Token(tok.clone()));
		}
		is_present
	}

	/// Consumes a token 'tok' if it exists. Returns whether the given token was present.
	pub fn eat(&mut self, tok: &TokenKind) -> bool {
		let is_present = self.check(tok);
		if is_present {
			self.bump()
		}
		is_present
	}

	/// If the next token is the given keyword, returns `true` without eating it.
	/// An expectation is also added for diagnostics purposes.
	fn check_keyword(&mut self, kw: Symbol) -> bool {
		// self.expected_tokens.push(TokenType::Keyword(kw));
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
