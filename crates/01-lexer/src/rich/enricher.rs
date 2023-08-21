use crate::{
	poor,
	rich::{AttrStyle, Delimiter, LiteralKind, OpKind, Token, TokenKind},
	span::Span,
	symbols::Symbol,
};

pub struct Enricher<'a> {
	source: &'a str,
	cursor: poor::Cursor<'a>,
	pos: u32,
}

impl<'a> Enricher<'a> {
	#[must_use]
	pub fn from_source(source: &'a str) -> Self {
		Self {
			source,
			cursor: poor::Cursor::new(source),
			pos: 0,
		}
	}

	pub fn next_token(&mut self) -> (Token, bool) {
		let mut has_whitespace_before = false;

		loop {
			let token = self.cursor.advance_token();
			let start = self.pos;
			self.pos += token.length;

			let kind = match token.kind {
				poor::TokenKind::Ident => {
					let ident = self.str_from(start).to_owned();
					// TODO: `oula` need to remove this a fast as possible
					let ident = Box::leak(Box::new(ident));

					let sym = Symbol::new_static(ident);
					TokenKind::Ident(sym)
				}
				poor::TokenKind::LineComment(style) => {
					let (style, sym) = self.cook_line_comment(style);

					TokenKind::Comment(style, sym)
				}
				poor::TokenKind::Literal(kind) => {
					let (kind, sym) = self.cook_literal(kind);

					TokenKind::Literal(kind, sym)
				}

				poor::TokenKind::Whitespace => {
					has_whitespace_before = true;
					continue;
				}

				poor::TokenKind::InvalidIdent | poor::TokenKind::Unknown => todo!(),

				poor::TokenKind::Semi => TokenKind::Semi,
				poor::TokenKind::Comma => TokenKind::Comma,
				poor::TokenKind::Dot => TokenKind::Dot,

				poor::TokenKind::OpenParenthesis => TokenKind::OpenDelim(Delimiter::Parenthesis),
				poor::TokenKind::CloseParenthesis => TokenKind::CloseDelim(Delimiter::Parenthesis),
				poor::TokenKind::OpenBrace => TokenKind::OpenDelim(Delimiter::Brace),
				poor::TokenKind::CloseBrace => TokenKind::CloseDelim(Delimiter::Brace),
				poor::TokenKind::OpenBracket => TokenKind::OpenDelim(Delimiter::Bracket),
				poor::TokenKind::CloseBracket => TokenKind::CloseDelim(Delimiter::Bracket),

				poor::TokenKind::At => TokenKind::At,
				poor::TokenKind::Pound => TokenKind::Pound,
				poor::TokenKind::Tilde => TokenKind::Tilde,
				poor::TokenKind::Question => TokenKind::Question,
				poor::TokenKind::Colon => TokenKind::Colon,
				poor::TokenKind::Dollar => TokenKind::Dollar,
				poor::TokenKind::Eq => TokenKind::Eq,
				poor::TokenKind::Bang => TokenKind::Bang,

				poor::TokenKind::Lt => TokenKind::Op(OpKind::Lt),
				poor::TokenKind::Gt => TokenKind::Op(OpKind::Gt),
				poor::TokenKind::Minus => TokenKind::Op(OpKind::Minus),
				poor::TokenKind::And => TokenKind::Op(OpKind::And),
				poor::TokenKind::Or => TokenKind::Op(OpKind::Or),
				poor::TokenKind::Plus => TokenKind::Op(OpKind::Plus),
				poor::TokenKind::Star => TokenKind::Op(OpKind::Star),
				poor::TokenKind::Slash => TokenKind::Op(OpKind::Slash),
				poor::TokenKind::Caret => TokenKind::Op(OpKind::Caret),
				poor::TokenKind::Percent => TokenKind::Op(OpKind::Percent),

				poor::TokenKind::Eof => TokenKind::Eof,
			};

			let span = Span::from_bounds(start, self.pos);
			return (Token::new(kind, span), has_whitespace_before);
		}
	}

	fn cook_line_comment(&mut self, style: Option<poor::DocStyle>) -> (AttrStyle, Symbol) {
		todo!()
	}

	fn cook_literal(&mut self, kind: poor::LiteralKind) -> (LiteralKind, Symbol) {
		todo!()
	}

	/// Slice of the source text from `start` up to but excluding `self.pos`,
	/// meaning the slice does not include the character `self.ch`.
	fn str_from(&self, start: u32) -> &'a str {
		self.str_from_to(start, self.pos)
	}

	/// Slice of the source text spanning from `start` up to but excluding `end`.
	fn str_from_to(&self, start: u32, end: u32) -> &'a str {
		&self.source[(start as usize)..(end as usize)]
	}
}

pub struct Iter<'a>(Enricher<'a>);

impl<'a> Iterator for Iter<'a> {
	type Item = Token;

	fn next(&mut self) -> Option<Self::Item> {
		let (token, _) = self.0.next_token();

		if token.kind.is_eof() {
			None
		} else {
			Some(token)
		}
	}
}

impl<'a> IntoIterator for Enricher<'a> {
	type Item = Token;
	type IntoIter = Iter<'a>;

	fn into_iter(self) -> Self::IntoIter {
		Iter(self)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		symbols::attrs,
		tests::{ATTR, EXAMPLE},
	};

	#[test]
	fn can_enrich_example_file() {
		let _rich_tokens = Enricher::from_source(EXAMPLE)
			.into_iter()
			.collect::<Vec<_>>();
	}

	#[test]
	fn parse_attr_style() {
		use crate::rich::TokenKind::*;
		use crate::span::Span;

		let mut tokens = Enricher::from_source(ATTR).into_iter();

		macro_rules! next_token {
			($ty:expr, $lo:literal, $hi:literal) => {
				assert_eq!(
					tokens.next(),
					Some(Token::new($ty, Span::from_bounds($lo, $hi)))
				);
			};
			(@end) => {
				assert_eq!(tokens.next(), None);
			};
		}

		next_token!(At, 0, 1);
		next_token!(Ident(attrs::Format), 1, 7);
		next_token!(Colon, 7, 8);
		next_token!(Ident(Symbol::new_static("date")), 9, 13);
		next_token!(@end);
	}
}
